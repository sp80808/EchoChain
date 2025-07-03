import asyncio
import json


class LocalAPI:
    def __init__(self, node, api_port):
        self.node = node
        self.api_port = api_port

    async def start(self):
        server = await asyncio.start_server(
            self.handle_connection, "127.0.0.1", self.api_port
        )
        print(f"Local API server started on 127.0.0.1:{self.api_port}")
        async with server:
            await server.serve_forever()

    async def handle_connection(self, reader, writer):
        addr = writer.get_extra_info("peername")
        try:
            data = await reader.read(4096)
            if not data:
                return
            message = json.loads(data.decode())
            response = await self.process_command(message)
            writer.write(json.dumps(response).encode())
            await writer.drain()
        except Exception as e:
            print(f"API error from {addr}: {e}")
        finally:
            writer.close()
            await writer.wait_closed()

    async def process_command(self, message):
        """
        Handle local API commands from clients (macOS app, Node.js, etc).
        Supported commands:
          - local_add_file
          - local_announce_content
          - local_request_content_info
          - local_request_file
          - local_list_content
          - local_register_file_on_chain
          - local_verify_file_on_chain
        """
        cmd_type = message.get("type")
        payload = message.get("payload", {})
        if cmd_type == "local_add_file":
            file_hash = self.node.file_manager.add_file(payload.get("filepath"))
            if file_hash:
                # Get audio analysis if available
                analysis = self.node.file_manager.get_audio_analysis(file_hash)
                response = {"status": "success", "file_hash": file_hash}

                # Include audio analysis in response
                if analysis and analysis.get("is_audio"):
                    response["audio_analysis"] = {
                        "detected_key": analysis.get("detected_key", "unknown"),
                        "tempo_bpm": analysis.get("tempo_bpm", 0.0),
                        "duration_seconds": analysis.get("duration_seconds", 0.0),
                        "audio_fingerprint": analysis.get("audio_fingerprint"),
                    }

                return response
            else:
                return {"status": "error", "message": "File not found"}
        elif cmd_type == "local_announce_content":
            content_hash = payload.get("content_hash")
            peer_id = self.node.peer_id or "unknown"
            self.node.dht.announce_content(content_hash, peer_id)
            return {"status": "success"}
        elif cmd_type == "local_request_content_info":
            content_hash = payload.get("content_hash")
            if content_hash == "all_available_content":
                available_content_list = []
                for file_hash, file_info in self.node.file_manager.files.items():
                    content_item = {
                        "content_id": file_hash,
                        "filename": file_info["filename"],
                        "size": file_info["size"],
                        "num_chunks": len(file_info.get("chunks", {})),
                        "peers": self.node.dht.get_peers_for_content(file_hash),
                    }

                    # Add audio analysis if available
                    analysis = file_info.get("analysis")
                    if analysis and analysis.get("is_audio"):
                        content_item["audio_analysis"] = {
                            "detected_key": analysis.get("detected_key", "unknown"),
                            "tempo_bpm": analysis.get("tempo_bpm", 0.0),
                            "duration_seconds": analysis.get("duration_seconds", 0.0),
                        }

                    available_content_list.append(content_item)
                return {
                    "status": "success",
                    "available_content": available_content_list,
                }
            else:
                peers = self.node.dht.get_peers_for_content(content_hash)
                if peers:
                    return {"status": "success", "peers": peers}
                else:
                    return {"status": "error", "message": "Content not found in DHT"}
        elif cmd_type == "local_request_file":
            content_hash = payload.get("content_hash")

            # First check if we already have the file locally
            if self.node.file_manager.has_file(content_hash):
                file_info = self.node.file_manager.get_file_info(content_hash)
                return {
                    "status": "success",
                    "message": "File already available locally",
                    "filename": file_info["filename"],
                }

            # Get peers that have this content
            peers = self.node.dht.get_peers_for_content(content_hash)
            if not peers:
                return {"status": "error", "message": "No peers found for this content"}

            # Get file info from a peer
            file_info = None
            for peer in peers:
                try:
                    request = {
                        "type": "request_file_info",
                        "payload": {"file_hash": content_hash},
                    }
                    response = await self.node.networking.send_message_to_peer(
                        peer, request
                    )
                    if response and response.get("status") == "success":
                        file_info = response.get("file_info")
                        break
                except Exception as e:
                    print(f"Error getting file info from peer {peer}: {e}")
                    continue

            if not file_info:
                return {
                    "status": "error",
                    "message": "Could not get file information from peers",
                }

            # Initiate download
            try:
                downloaded_path = await self.node.file_manager.download_file_from_peers(
                    content_hash, file_info, peers, self.node.networking
                )

                if downloaded_path:
                    return {
                        "status": "success",
                        "message": "File downloaded successfully",
                        "file_path": downloaded_path,
                        "filename": file_info.get("filename", "unknown"),
                    }
                else:
                    return {"status": "error", "message": "Download failed"}

            except Exception as e:
                return {"status": "error", "message": f"Download error: {str(e)}"}
        elif cmd_type == "local_list_content":
            all_content = self.node.dht.get_all_content()
            return {"status": "success", "content_hashes": all_content}
        elif cmd_type == "local_register_file_on_chain":
            file_hash = payload.get("file_hash")
            owner_mnemonic = payload.get("owner_mnemonic")
            metadata = payload.get("metadata", {})
            result = self.node.blockchain.register_file_on_chain(
                file_hash, owner_mnemonic, metadata
            )
            return {"status": "success" if result else "error"}
        elif cmd_type == "local_verify_file_on_chain":
            file_hash = payload.get("file_hash")
            owner_address = payload.get("owner_address")
            result = self.node.blockchain.verify_file_on_chain(file_hash, owner_address)
            return {"status": "success" if result else "error"}
        elif cmd_type == "local_audio_analysis":
            file_hash = payload.get("file_hash")
            analysis = self.node.file_manager.get_audio_analysis(file_hash)
            if analysis:
                return {
                    "status": "success",
                    "file_hash": file_hash,
                    "analysis": analysis,
                }
            else:
                return {"status": "error", "message": "No analysis found for this file"}
        elif cmd_type == "local_find_by_key":
            target_key = payload.get("key")
            tolerance = payload.get("tolerance", 2)
            if not target_key:
                return {"status": "error", "message": "Missing key parameter"}

            try:
                compatible_files = self.node.file_manager.find_audio_files_by_key(
                    target_key, tolerance
                )
                return {
                    "status": "success",
                    "target_key": target_key,
                    "tolerance": tolerance,
                    "compatible_files": compatible_files,
                    "count": len(compatible_files),
                }
            except Exception as e:
                return {"status": "error", "message": str(e)}
        elif cmd_type == "local_audio_summary":
            try:
                summary = self.node.file_manager.get_audio_files_summary()
                return {"status": "success", "summary": summary}
            except Exception as e:
                return {"status": "error", "message": str(e)}
        elif cmd_type == "local_search_audio":
            search_params = payload
            try:
                # Get all audio files
                summary = self.node.file_manager.get_audio_files_summary()
                audio_files = summary.get("audio_files", [])

                # Apply filters
                filtered_files = audio_files

                # Filter by key
                if "key" in search_params and search_params["key"]:
                    target_key = search_params["key"]
                    tolerance = search_params.get("key_tolerance", 2)
                    compatible_files = self.node.file_manager.find_audio_files_by_key(
                        target_key, tolerance
                    )
                    compatible_hashes = {cf["file_hash"] for cf in compatible_files}
                    filtered_files = [
                        f for f in filtered_files if f["file_hash"] in compatible_hashes
                    ]

                # Filter by tempo range
                if "min_tempo" in search_params or "max_tempo" in search_params:
                    min_tempo = search_params.get("min_tempo", 0)
                    max_tempo = search_params.get("max_tempo", 999)
                    filtered_files = [
                        f
                        for f in filtered_files
                        if min_tempo <= f.get("tempo_bpm", 0) <= max_tempo
                    ]

                # Filter by duration range
                if "min_duration" in search_params or "max_duration" in search_params:
                    min_duration = search_params.get("min_duration", 0)
                    max_duration = search_params.get("max_duration", 99999)
                    filtered_files = [
                        f
                        for f in filtered_files
                        if min_duration <= f.get("duration_seconds", 0) <= max_duration
                    ]

                return {
                    "status": "success",
                    "search_criteria": search_params,
                    "results": filtered_files,
                    "count": len(filtered_files),
                }
            except Exception as e:
                return {"status": "error", "message": str(e)}
        return {"status": "error", "message": "Unknown command"}
