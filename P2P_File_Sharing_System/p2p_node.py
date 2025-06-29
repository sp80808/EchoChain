import asyncio
import json
import hashlib
import os
import uuid
from collections import defaultdict

class P2PNode:
    def __init__(self, host, port, local_api_port):
        self.host = host
        self.port = port
        self.local_api_port = local_api_port
        self.peer_id = str(uuid.uuid4()) # Unique ID for this node
        self.peers = {}  # {peer_id: (host, port)}
        self.dht = {}  # {content_hash: [peer_id1, peer_id2, ...]} - Simplified DHT for now
        self.files = {}  # {file_hash: {'filename': str, 'size': int, 'chunks': {chunk_index: chunk_data}}}
        self.chunk_size = 1024 * 1024  # 1 MB
        self.downloading_files = defaultdict(lambda: {'metadata': None, 'received_chunks': {}, 'peers_with_chunks': defaultdict(list)})
        self.uploading_files = {} # {file_hash: {peer_id: [chunks_to_send]}}

    async def start(self):
        server = await asyncio.start_server(
            self.handle_connection, self.host, self.port
        )
        addr = server.sockets[0].getsockname()
        print(f"Serving P2P on {addr} with Peer ID: {self.peer_id}")
        
        # Start local API server
        asyncio.create_task(self.start_local_api_server())

        async with server:
            await server.serve_forever()

    async def handle_connection(self, reader, writer):
        addr = writer.get_extra_info('peername')
        print(f"Connected by {addr}")
        try:
            while True:
                data = await reader.read(4096)
                if not data:
                    break
                message = json.loads(data.decode())
                await self.process_message(message, writer)
        except Exception as e:
            print(f"Error handling connection from {addr}: {e}")
        finally:
            print(f"Client disconnected: {addr}")
            writer.close()
            await writer.wait_closed()

    async def send_message(self, peer_host, peer_port, message):
        try:
            reader, writer = await asyncio.open_connection(peer_host, peer_port)
            writer.write(json.dumps(message).encode())
            await writer.drain()
            writer.close()
            await writer.wait_closed()
        except Exception as e:
            print(f"Could not send message to {peer_host}:{peer_port}: {e}")

    async def process_message(self, message, writer):
        msg_type = message.get('type')
        payload = message.get('payload')
        sender_addr = writer.get_extra_info('peername')
        sender_peer_id = payload.get('sender_id') # Assuming sender_id is always included

        if sender_peer_id and sender_peer_id not in self.peers and (sender_addr[0], sender_addr[1]) != (self.host, self.port):
            self.peers[sender_peer_id] = (sender_addr[0], sender_addr[1])
            print(f"Discovered new peer: {sender_peer_id} at {sender_addr[0]}:{sender_addr[1]}")

        if msg_type == 'discover_peers':
            # Respond with known peers and our own ID
            response = {'type': 'peer_list', 'payload': {'peers': self.peers, 'your_id': self.peer_id}}
            writer.write(json.dumps(response).encode())
            await writer.drain()
        elif msg_type == 'peer_list':
            # Add new peers to our list
            for peer_id, (host, port) in payload['peers'].items():
                if peer_id not in self.peers and (host, port) != (self.host, self.port):
                    self.peers[peer_id] = (host, port)
                    print(f"Discovered new peer: {peer_id} at {host}:{port}")
        elif msg_type == 'announce_content':
            content_hash = payload['content_hash']
            peer_id = payload['peer_id']
            if content_hash not in self.dht:
                self.dht[content_hash] = []
            if peer_id not in self.dht[content_hash]:
                self.dht[content_hash].append(peer_id)
            print(f"Content {content_hash} announced by {peer_id}")
        elif msg_type == 'request_content_info':
            content_hash = payload['content_hash']
            if content_hash in self.files:
                file_info = {
                    'filename': self.files[content_hash]['filename'],
                    'size': self.files[content_hash]['size'],
                    'num_chunks': len(self.files[content_hash]['chunks']),
                    'chunk_hashes': [hashlib.sha256(chunk).hexdigest() for chunk in self.files[content_hash]['chunks'].values()]
                }
                response = {'type': 'content_info', 'payload': {'content_hash': content_hash, 'file_info': file_info}}
                writer.write(json.dumps(response).encode())
                await writer.drain()
            else:
                response = {'type': 'error', 'payload': 'Content not found'}
                writer.write(json.dumps(response).encode())
                await writer.drain()
        elif msg_type == 'request_chunk':
            content_hash = payload['content_hash']
            chunk_index = payload['chunk_index']
            if content_hash in self.files and chunk_index in self.files[content_hash]['chunks']:
                chunk_data = self.files[content_hash]['chunks'][chunk_index]
                response = {'type': 'chunk_data', 'payload': {'content_hash': content_hash, 'chunk_index': chunk_index, 'data': chunk_data.hex()}}
                writer.write(json.dumps(response).encode())
                await writer.drain()
            else:
                response = {'type': 'error', 'payload': 'Chunk not found'}
                writer.write(json.dumps(response).encode())
                await writer.drain()
        elif msg_type == 'chunk_data':
            content_hash = payload['content_hash']
            chunk_index = payload['chunk_index']
            chunk_data = bytes.fromhex(payload['data'])
            chunk_hash = hashlib.sha256(chunk_data).hexdigest()

            if content_hash not in self.downloading_files:
                print(f"Received unexpected chunk {chunk_index} for unknown download {content_hash}")
                return

            expected_hash = self.downloading_files[content_hash]['metadata']['chunk_hashes'][chunk_index]
            if chunk_hash != expected_hash:
                print(f"Chunk {chunk_index} for {content_hash} failed hash verification. Expected {expected_hash}, got {chunk_hash}")
                # Potentially request chunk again from another peer
                return

            self.downloading_files[content_hash]['received_chunks'][chunk_index] = chunk_data
            print(f"Received and verified chunk {chunk_index} for {content_hash}")

            # Check if download is complete
            num_expected_chunks = self.downloading_files[content_hash]['metadata']['num_chunks']
            if len(self.downloading_files[content_hash]['received_chunks']) == num_expected_chunks:
                print(f"All chunks received for {content_hash}. Reassembling file...")
                await self._reassemble_file(content_hash)
        else:
            print(f"Unknown message type: {msg_type}")

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
            return

        filename = os.path.basename(filepath)
        file_size = os.path.getsize(filepath)
        file_chunks = {}
        
        file_hash = self._calculate_file_hash(filepath) # Hash based on content

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

    async def discover_peers(self, target_host, target_port):
        message = {'type': 'discover_peers', 'payload': {'sender_id': self.peer_id}}
        await self.send_message(target_host, target_port, message)

    async def announce_content(self, content_hash):
        message = {'type': 'announce_content', 'payload': {'content_hash': content_hash, 'peer_id': self.peer_id, 'sender_id': self.peer_id}}
        # Announce to all known peers
        for peer_id, (host, port) in list(self.peers.items()): # Use list to avoid RuntimeError during iteration if self.peers changes
            await self.send_message(host, port, message)
        # Also add to our own DHT
        if content_hash not in self.dht:
            self.dht[content_hash] = []
        if self.peer_id not in self.dht[content_hash]:
            self.dht[content_hash].append(self.peer_id)

    async def request_content_info(self, peer_host, peer_port, content_hash):
        message = {'type': 'request_content_info', 'payload': {'content_hash': content_hash, 'sender_id': self.peer_id}}
        await self.send_message(peer_host, peer_port, message)

    async def request_chunk(self, peer_host, peer_port, content_hash, chunk_index):
        message = {'type': 'request_chunk', 'payload': {'content_hash': content_hash, 'chunk_index': chunk_index, 'sender_id': self.peer_id}}
        await self.send_message(peer_host, peer_port, message)

async def main():
    # Example usage:
    # Node 1
    node1 = P2PNode('127.0.0.1', 8000, 8002) # P2P port 8000, Local API port 8002

    # Node 2
    node2 = P2PNode('127.0.0.1', 8001, 8003) # P2P port 8001, Local API port 8003

    # Start nodes in the background
    asyncio.create_task(node1.start())
    asyncio.create_task(node2.start())

    await asyncio.sleep(1) # Give servers time to start

    # Node 1 discovers Node 2
    print(f"Node 1 ({node1.peer_id}) discovering peers...")
    await node1.discover_peers('127.0.0.1', 8001)
    await asyncio.sleep(1) # Give time for peer list to update

    # Node 2 discovers Node 1
    print(f"Node 2 ({node2.peer_id}) discovering peers...")
    await node2.discover_peers('127.0.0.1', 8000)
    await asyncio.sleep(1) # Give time for peer list to update

    print(f"\nNode 1 Peers ({node1.peer_id}):", node1.peers)
    print(f"Node 2 Peers ({node2.peer_id}):", node2.peers)

    # Example: Node 1 adds a dummy file and announces it
    dummy_file_path = "P2P_File_Sharing_System/dummy_sample.txt"
    with open(dummy_file_path, "w") as f:
        f.write("This is a dummy music sample file for testing the P2P system. " * 100)
    
    node1_file_hash = node1.add_file(dummy_file_path)
    if node1_file_hash:
        print(f"\nNode 1 ({node1.peer_id}) announcing content {node1_file_hash}...")
        await node1.announce_content(node1_file_hash)
        await asyncio.sleep(1)
        print(f"\nNode 1 DHT ({node1.peer_id}):", node1.dht)
        print(f"Node 2 DHT ({node2.peer_id}):", node2.dht) # Node 2 should have received the announcement

        # Example: Node 2 requests content info and then initiates download from Node 1
        if node1_file_hash in node2.dht:
            print(f"\nNode 2 ({node2.peer_id}) initiating download for {node1_file_hash}...")
            await node2._initiate_file_download(node1_file_hash)
            await asyncio.sleep(5) # Give time for download to complete

            # Verify if Node 2 has the file
            downloaded_filepath = os.path.join("downloads", node1.files[node1_file_hash]['filename'])
            if os.path.exists(downloaded_filepath):
                print(f"\nNode 2 successfully downloaded and reassembled: {downloaded_filepath}")
                # Optional: Verify hash of downloaded file
                downloaded_hash = node2._calculate_file_hash(downloaded_filepath)
                if downloaded_hash == node1_file_hash:
                    print(f"Downloaded file hash matches original: {downloaded_hash}")
                else:
                    print(f"Downloaded file hash MISMATCH! Expected {node1_file_hash}, got {downloaded_hash}")
            else:
                print(f"\nNode 2 did NOT successfully download {node1_file_hash}.")

    # Keep the event loop running for a while to observe interactions
    await asyncio.sleep(10)

    async def start_local_api_server(self):
        local_server = await asyncio.start_server(
            self.handle_local_api_connection, self.host, self.local_api_port
        )
        local_addr = local_server.sockets[0].getsockname()
        print(f"Local API serving on {local_addr}")
        async with local_server:
            await local_server.serve_forever()

    async def handle_local_api_connection(self, reader, writer):
        addr = writer.get_extra_info('peername')
        print(f"Local API connected by {addr}")
        try:
            while True:
                data = await reader.read(4096)
                if not data:
                    break
                message = json.loads(data.decode())
                response = await self.process_local_command(message)
                writer.write(json.dumps(response).encode())
                await writer.drain()
        except Exception as e:
            print(f"Error handling local API connection from {addr}: {e}")
        finally:
            print(f"Local API client disconnected: {addr}")
            writer.close()
            await writer.wait_closed()

    async def process_local_command(self, message):
        cmd_type = message.get('type')
        payload = message.get('payload')

        if cmd_type == 'local_add_file':
            filepath = payload['filepath']
            file_hash = self.add_file(filepath)
            if file_hash:
                return {'status': 'success', 'file_hash': file_hash}
            else:
                return {'status': 'error', 'message': 'Failed to add file'}
        elif cmd_type == 'local_announce_content':
            content_hash = payload['content_hash']
            await self.announce_content(content_hash)
            return {'status': 'success'}
        elif cmd_type == 'local_request_content_info':
            content_hash = payload['content_hash']
            # This needs to be a blocking call or return a future/callback
            # For simplicity, we'll just return the DHT info we have
            # A real implementation would involve waiting for P2P response
            if content_hash in self.dht:
                return {'status': 'success', 'peers': self.dht[content_hash]}
            else:
                return {'status': 'error', 'message': 'Content not found in DHT'}
        elif cmd_type == 'local_request_file':
            content_hash = payload['content_hash']
            # This is a complex operation that involves requesting chunks from peers
            # For now, just a placeholder. A real implementation would manage the download.
            print(f"Local API received request to download {content_hash}. (Download logic to be implemented)")
            return {'status': 'success', 'message': 'Download initiated (placeholder)'}
        else:
            return {'status': 'error', 'message': f"Unknown local command: {cmd_type}"}

    async def _reassemble_file(self, content_hash):
        if content_hash not in self.downloading_files or not self.downloading_files[content_hash]['metadata']:
            print(f"Cannot reassemble file: metadata missing for {content_hash}")
            return

        file_metadata = self.downloading_files[content_hash]['metadata']
        received_chunks = self.downloading_files[content_hash]['received_chunks']
        
        if len(received_chunks) != file_metadata['num_chunks']:
            print(f"Not all chunks received for {content_hash}. Received {len(received_chunks)} of {file_metadata['num_chunks']}")
            return

        output_filepath = os.path.join("downloads", file_metadata['filename'])
        os.makedirs(os.path.dirname(output_filepath), exist_ok=True)

        try:
            with open(output_filepath, 'wb') as f:
                for i in range(file_metadata['num_chunks']):
                    f.write(received_chunks[i])
            print(f"File {file_metadata['filename']} reassembled successfully at {output_filepath}")
            # Clean up download state
            del self.downloading_files[content_hash]
        except Exception as e:
            print(f"Error reassembling file {file_metadata['filename']}: {e}")

    async def start_local_api_server(self):
        local_server = await asyncio.start_server(
            self.handle_local_api_connection, self.host, self.local_api_port
        )
        local_addr = local_server.sockets[0].getsockname()
        print(f"Local API serving on {local_addr}")
        async with local_server:
            await local_server.serve_forever()

    async def handle_local_api_connection(self, reader, writer):
        addr = writer.get_extra_info('peername')
        print(f"Local API connected by {addr}")
        try:
            while True:
                data = await reader.read(4096)
                if not data:
                    break
                message = json.loads(data.decode())
                response = await self.process_local_command(message)
                writer.write(json.dumps(response).encode())
                await writer.drain()
        except Exception as e:
            print(f"Error handling local API connection from {addr}: {e}")
        finally:
            print(f"Local API client disconnected: {addr}")
            writer.close()
            await writer.wait_closed()

    async def process_local_command(self, message):
        cmd_type = message.get('type')
        payload = message.get('payload')

        if cmd_type == 'local_add_file':
            filepath = payload['filepath']
            file_hash = self.add_file(filepath)
            if file_hash:
                return {'status': 'success', 'file_hash': file_hash}
            else:
                return {'status': 'error', 'message': 'Failed to add file'}
        elif cmd_type == 'local_announce_content':
            content_hash = payload['content_hash']
            await self.announce_content(content_hash)
            return {'status': 'success'}
        elif cmd_type == 'local_request_content_info':
            content_hash = payload['content_hash']
            if content_hash == "all_available_content":
                available_content_list = []
                for file_hash, file_info in self.files.items():
                    # For simplicity, we're returning basic info.
                    # In a real scenario, this might involve querying the blockchain for richer metadata.
                    available_content_list.append({
                        'content_id': file_hash,
                        'filename': file_info['filename'],
                        'size': file_info['size'],
                        'num_chunks': len(file_info['chunks']),
                        'peers': self.dht.get(file_hash, []) # Peers that announced this content
                    })
                return {'status': 'success', 'available_content': available_content_list}
            elif content_hash in self.dht:
                return {'status': 'success', 'peers': self.dht[content_hash]}
            else:
                return {'status': 'error', 'message': 'Content not found in DHT'}
        elif cmd_type == 'local_request_file':
            content_hash = payload['content_hash']
            print(f"Local API received request to download {content_hash}.")
            
            # Initiate download process
            asyncio.create_task(self._initiate_file_download(content_hash))
            return {'status': 'success', 'message': 'Download initiated'}
        else:
            return {'status': 'error', 'message': f"Unknown local command: {cmd_type}"}

    async def _initiate_file_download(self, content_hash):
        if content_hash in self.downloading_files and self.downloading_files[content_hash]['metadata']:
            print(f"Download for {content_hash} already in progress.")
            return

        # 1. Discover peers that have the content
        if content_hash not in self.dht or not self.dht[content_hash]:
            print(f"No peers found for content hash {content_hash} in DHT.")
            return {'status': 'error', 'message': 'No peers found for content'}

        candidate_peers = self.dht[content_hash]
        if not candidate_peers:
            print(f"No peers available for content hash {content_hash}.")
            return {'status': 'error', 'message': 'No peers available'}

        # 2. Request content info from a peer
        # For simplicity, pick the first peer. In a real system, you'd try multiple.
        peer_id_to_request = candidate_peers[0]
        if peer_id_to_request not in self.peers:
            print(f"Peer {peer_id_to_request} not in our active peer list.")
            return {'status': 'error', 'message': 'Peer not active'}

        peer_host, peer_port = self.peers[peer_id_to_request]
        
        # Send request_content_info and wait for response
        # This requires a more sophisticated send_message that waits for a response
        # For now, we'll simulate or assume the info is received via process_message
        # A better approach would be to use a Future or a callback mechanism.
        print(f"Requesting content info for {content_hash} from {peer_id_to_request}")
        
        # Simulate waiting for content_info response
        # In a real system, this would be a blocking call or use a Future
        # For now, we'll rely on the content_info message being processed by handle_connection
        # and updating self.downloading_files[content_hash]['metadata']
        await self.send_message(peer_host, peer_port, {'type': 'request_content_info', 'payload': {'content_hash': content_hash, 'sender_id': self.peer_id}})
        
        # Wait for a short period for the content_info to arrive and be processed
        await asyncio.sleep(2)

        if content_hash not in self.downloading_files or not self.downloading_files[content_hash]['metadata']:
            print(f"Failed to get content metadata for {content_hash}.")
            return {'status': 'error', 'message': 'Failed to get content metadata'}

        file_metadata = self.downloading_files[content_hash]['metadata']
        num_chunks = file_metadata['num_chunks']
        print(f"Initiating download for {file_metadata['filename']} ({num_chunks} chunks)")

        # 3. Request chunks in parallel
        # This is a simplified parallel download. In a real system, you'd manage
        # active connections, peer selection, and retry logic.
        chunk_requests = []
        for i in range(num_chunks):
            chunk_requests.append(self.request_chunk(peer_host, peer_port, content_hash, i))
        
        # Await all chunk requests. This is not robust for large files or unreliable networks.
        # A proper implementation would manage individual chunk downloads and retries.
        await asyncio.gather(*chunk_requests)

        # The _reassemble_file method is called when all chunks are received in process_message
        print(f"Download process for {content_hash} completed (all chunks requested).")

    async def request_chunk(self, peer_host, peer_port, content_hash, chunk_index):
        message = {'type': 'request_chunk', 'payload': {'content_hash': content_hash, 'chunk_index': chunk_index, 'sender_id': self.peer_id}}
        await self.send_message(peer_host, peer_port, message)

if __name__ == "__main__":
    asyncio.run(main())