import os
import hashlib
import asyncio
import json
from typing import Optional, Dict, Any

# Import audio analysis module if available
try:
    from .audio_analysis import analyze_audio_for_p2p

    AUDIO_ANALYSIS_AVAILABLE = True
except ImportError:
    AUDIO_ANALYSIS_AVAILABLE = False
    print(
        "Audio analysis module not available. Install librosa for audio analysis features."
    )


class FileManager:
    def __init__(self, chunk_size=1024 * 1024):
        self.files = (
            {}
        )  # {file_hash: {'filename': str, 'size': int, 'chunks': {chunk_index: chunk_data}, 'analysis': dict}}
        self.chunk_size = chunk_size
        self.download_dir = "downloads"  # Directory for downloaded files
        self.analysis_cache = {}  # Cache for audio analysis results

        # Ensure download directory exists
        os.makedirs(self.download_dir, exist_ok=True)

    def _calculate_file_hash(self, filepath):
        hasher = hashlib.sha256()
        with open(filepath, "rb") as f:
            while True:
                chunk = f.read(self.chunk_size)
                if not chunk:
                    break
                hasher.update(chunk)
        return hasher.hexdigest()

    def add_file(self, filepath):
        if not os.path.exists(filepath):
            print(f"File not found: {filepath}")
            return None
        filename = os.path.basename(filepath)
        file_size = os.path.getsize(filepath)
        file_chunks = {}
        file_hash = self._calculate_file_hash(filepath)

        with open(filepath, "rb") as f:
            chunk_index = 0
            while True:
                chunk = f.read(self.chunk_size)
                if not chunk:
                    break
                file_chunks[chunk_index] = chunk
                chunk_index += 1

        # Perform audio analysis if the file is an audio file
        analysis_result = self._analyze_file_if_audio(filepath)

        self.files[file_hash] = {
            "filename": filename,
            "size": file_size,
            "chunks": file_chunks,
            "analysis": analysis_result,
            "num_chunks": len(file_chunks),
        }

        # Cache analysis result
        if analysis_result and analysis_result.get("is_audio"):
            self.analysis_cache[file_hash] = analysis_result

        print(
            f"Added file {filename} with hash {file_hash} and {len(file_chunks)} chunks."
        )
        if analysis_result and analysis_result.get("is_audio"):
            tempo = analysis_result.get("tempo_bpm", 0.0)
            print(f"  Audio analysis: Tempo={tempo} BPM")

        return file_hash

    def get_file_info(self, file_hash):
        return self.files.get(file_hash)

    def get_chunk(self, file_hash, chunk_index):
        """Get a specific chunk of a file by its hash and chunk index."""
        file_info = self.files.get(file_hash)
        if file_info and chunk_index in file_info["chunks"]:
            return file_info["chunks"][chunk_index]
        return None

    def has_file(self, file_hash):
        """Check if file exists in local storage."""
        return file_hash in self.files

    async def download_file_from_peers(self, file_hash, file_info, peers, networking):
        """
        Download a file from available peers by requesting chunks.
        Returns the path to the downloaded file or None if failed.
        """
        if not peers:
            print(f"No peers available for file {file_hash}")
            return None

        filename = file_info.get("filename", f"file_{file_hash[:8]}")
        file_path = os.path.join(self.download_dir, filename)
        num_chunks = file_info.get("num_chunks", 0)

        if num_chunks == 0:
            print(f"Invalid file info for {file_hash}")
            return None

        downloaded_chunks = {}

        # Download chunks from peers
        for chunk_index in range(num_chunks):
            chunk_data = await self._download_chunk_from_peers(
                file_hash, chunk_index, peers, networking
            )
            if chunk_data:
                downloaded_chunks[chunk_index] = chunk_data
            else:
                print(f"Failed to download chunk {chunk_index} for file {file_hash}")
                return None

        # Reconstruct file from chunks
        try:
            with open(file_path, "wb") as f:
                for i in range(num_chunks):
                    if i in downloaded_chunks:
                        f.write(downloaded_chunks[i])
                    else:
                        print(f"Missing chunk {i} for file {file_hash}")
                        return None

            # Verify file integrity
            if self._calculate_file_hash(file_path) == file_hash:
                print(f"Successfully downloaded file {filename}")

                # Add to local file store
                self.add_file(file_path)
                return file_path
            else:
                print(f"File integrity check failed for {filename}")
                os.remove(file_path)
                return None

        except Exception as e:
            print(f"Error reconstructing file {filename}: {e}")
            if os.path.exists(file_path):
                os.remove(file_path)
            return None

    async def _download_chunk_from_peers(
        self, file_hash, chunk_index, peers, networking
    ):
        """
        Try to download a specific chunk from available peers.
        """
        for peer in peers:
            try:
                # Send chunk request to peer
                request = {
                    "type": "request_chunk",
                    "payload": {"file_hash": file_hash, "chunk_index": chunk_index},
                }

                response = await networking.send_message_to_peer(peer, request)
                if response and response.get("status") == "success":
                    chunk_data = response.get("chunk_data")
                    if chunk_data:
                        # Convert from base64 if needed
                        import base64

                        try:
                            return base64.b64decode(chunk_data)
                        except:
                            # If not base64, assume it's raw data
                            return (
                                chunk_data.encode()
                                if isinstance(chunk_data, str)
                                else chunk_data
                            )

            except Exception as e:
                print(f"Error downloading chunk {chunk_index} from peer {peer}: {e}")
                continue

        return None

    def _analyze_file_if_audio(self, filepath: str) -> Optional[Dict[str, Any]]:
        """
        Analyze file if it's an audio file. Returns analysis results or None.
        """
        if not AUDIO_ANALYSIS_AVAILABLE:
            return None

        try:
            analysis_result = analyze_audio_for_p2p(filepath)
            return analysis_result
        except Exception as e:
            print(f"Audio analysis failed for {filepath}: {e}")
            return None

    def get_audio_analysis(self, file_hash: str) -> Optional[Dict[str, Any]]:
        """
        Get audio analysis results for a file.
        """
        file_info = self.files.get(file_hash)
        if file_info:
            return file_info.get("analysis")
        return self.analysis_cache.get(file_hash)

    def serve_chunk(self, file_hash, chunk_index):
        """
        Serve a chunk to a requesting peer.
        Returns base64 encoded chunk data or None if not available.
        """
        chunk_data = self.get_chunk(file_hash, chunk_index)
        if chunk_data:
            import base64

            return base64.b64encode(chunk_data).decode("utf-8")
        return None

    def handle_request_file_info(self, message):
        """
        Handle requests for file information including audio analysis.
        """
        file_hash = message.get("file_hash")
        if not file_hash:
            return {"status": "error", "message": "Missing file_hash"}

        file_info = self.get_file_info(file_hash)
        if not file_info:
            return {"status": "error", "message": "File not found"}

        # Prepare response with basic file info
        response = {
            "status": "success",
            "file_hash": file_hash,
            "filename": file_info["filename"],
            "size": file_info["size"],
            "num_chunks": file_info.get("num_chunks", len(file_info.get("chunks", {}))),
        }

        # Add audio analysis if available
        analysis = file_info.get("analysis")
        if analysis and analysis.get("is_audio"):
            response["audio_analysis"] = {
                "tempo_bpm": analysis.get("tempo_bpm", 0.0),
                "duration_seconds": analysis.get("duration_seconds", 0.0),
                "audio_fingerprint": analysis.get("audio_fingerprint"),
            }

        return response

    def handle_request_file_download(self, message):
        """
        Handle file download requests.
        """
        file_hash = message.get("file_hash")

        if not file_hash:
            return {"status": "error", "message": "Missing file_hash"}

        file_info = self.get_file_info(file_hash)
        if not file_info:
            return {"status": "error", "message": "File not found"}

        response = {
            "status": "success",
            "file_hash": file_hash,
            "filename": file_info["filename"],
            "size": file_info["size"],
            "num_chunks": file_info.get("num_chunks", len(file_info.get("chunks", {}))),
        }

        return response
