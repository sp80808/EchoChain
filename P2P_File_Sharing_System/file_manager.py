import os
import hashlib

class FileManager:
    def __init__(self, chunk_size=1024*1024):
        self.files = {}  # {file_hash: {'filename': str, 'size': int, 'chunks': {chunk_index: chunk_data}}}
        self.chunk_size = chunk_size

    def _calculate_file_hash(self, filepath):
        hasher = hashlib.sha256()
        with open(filepath, 'rb') as f:
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
        with open(filepath, 'rb') as f:
            chunk_index = 0
            while True:
                chunk = f.read(self.chunk_size)
                if not chunk:
                    break
                file_chunks[chunk_index] = chunk
                chunk_index += 1
        self.files[file_hash] = {
            'filename': filename,
            'size': file_size,
            'chunks': file_chunks
        }
        print(f"Added file {filename} with hash {file_hash} and {len(file_chunks)} chunks.")
        return file_hash

    def get_file_info(self, file_hash):
        return self.files.get(file_hash) 