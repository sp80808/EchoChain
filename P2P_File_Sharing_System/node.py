import asyncio
from networking import Networking
from dht import DHT
from file_manager import FileManager
from api import LocalAPI
import blockchain
import json

class EchoChainNode:
    """
    Main orchestrator for the EchoChain P2P node.
    Wires together networking, DHT, file management, local API, and blockchain integration.
    """
    def __init__(self, host, p2p_port, api_port):
        self.host = host
        self.p2p_port = p2p_port
        self.api_port = api_port
        self.peer_id = None  # To be set/assigned

        # Core modules
        self.dht = DHT()
        self.file_manager = FileManager()
        self.networking = Networking(self)
        self.api = LocalAPI(self, api_port)
        self.blockchain = blockchain  # Module, not instance

    async def start(self):
        """Start the P2P networking server and the local API server."""
        await asyncio.gather(
            self.networking.start_server(self.host, self.p2p_port),
            self.api.start()
        )

    async def process_message(self, message, writer):
        """
        Entry point for handling incoming P2P messages.
        Delegates to appropriate modules (DHT, FileManager, etc).
        Handles the following P2P message types:
          - announce_content
          - request_content_info
          - request_file_info
          - request_file_download
        """
        msg_type = message.get('type')
        payload = message.get('payload', {})
        response = {'status': 'error', 'message': 'Unknown command'}

        if msg_type == 'announce_content':
            content_hash = payload.get('content_hash')
            peer_id = payload.get('peer_id', self.peer_id or 'unknown')
            self.dht.announce_content(content_hash, peer_id)
            response = {'status': 'success'}

        elif msg_type == 'request_content_info':
            content_hash = payload.get('content_hash')
            if content_hash == 'all_available_content':
                available_content_list = []
                for file_hash, file_info in self.file_manager.files.items():
                    available_content_list.append({
                        'content_id': file_hash,
                        'filename': file_info['filename'],
                        'size': file_info['size'],
                        'num_chunks': len(file_info['chunks']),
                        'peers': self.dht.get_peers_for_content(file_hash)
                    })
                response = {'status': 'success', 'available_content': available_content_list}
            else:
                peers = self.dht.get_peers_for_content(content_hash)
                if peers:
                    response = {'status': 'success', 'peers': peers}
                else:
                    response = {'status': 'error', 'message': 'Content not found in DHT'}

        elif msg_type == 'request_file_info':
            file_hash = payload.get('file_hash')
            file_info = self.file_manager.get_file_info(file_hash)
            if file_info:
                response = {'status': 'success', 'file_info': {
                    'filename': file_info['filename'],
                    'size': file_info['size'],
                    'num_chunks': len(file_info['chunks'])
                }}
            else:
                response = {'status': 'error', 'message': 'File not found'}

        elif msg_type == 'request_file_download':
            content_hash = payload.get('content_hash')
            # TODO: Implement actual file transfer logic (stub for now)
            response = {'status': 'success', 'message': 'Download initiated (stub)'}

        # Write response to peer
        try:
            writer.write(json.dumps(response).encode())
            await writer.drain()
        except Exception as e:
            print(f"Error sending response to peer: {e}")

    # Add more orchestrator methods as needed for coordination 