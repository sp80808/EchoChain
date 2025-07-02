import asyncio
import json

class PeerConnection:
    def __init__(self, host, port, peer_id=None):
        self.host = host
        self.port = port
        self.peer_id = peer_id

    async def send_message(self, message):
        try:
            reader, writer = await asyncio.open_connection(self.host, self.port)
            writer.write(json.dumps(message).encode())
            await writer.drain()
            writer.close()
            await writer.wait_closed()
        except Exception as e:
            print(f"Could not send message to {self.host}:{self.port}: {e}")

class Networking:
    def __init__(self, node):
        self.node = node
        self.peers = {}  # {peer_id: (host, port)}

    async def start_server(self, host, port):
        server = await asyncio.start_server(self.handle_connection, host, port)
        print(f"Networking server started on {host}:{port}")
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
                await self.node.process_message(message, writer)
        except Exception as e:
            print(f"Error handling connection from {addr}: {e}")
        finally:
            print(f"Client disconnected: {addr}")
            writer.close()
            await writer.wait_closed()

    def add_peer(self, peer_id, host, port):
        self.peers[peer_id] = (host, port)

    def get_peers(self):
        return self.peers
    
    async def send_message_to_peer(self, peer_info, message):
        """
        Send a message to a specific peer and return the response.
        peer_info can be either a peer_id or a (host, port) tuple.
        """
        try:
            if isinstance(peer_info, str) and peer_info in self.peers:
                # peer_info is a peer_id
                host, port = self.peers[peer_info]
            elif isinstance(peer_info, (tuple, list)) and len(peer_info) == 2:
                # peer_info is (host, port)
                host, port = peer_info
            else:
                print(f"Invalid peer info: {peer_info}")
                return None
                
            reader, writer = await asyncio.open_connection(host, port)
            
            # Send message
            writer.write(json.dumps(message).encode())
            await writer.drain()
            
            # Read response
            response_data = await reader.read(4096)
            if response_data:
                response = json.loads(response_data.decode())
                return response
            
            return None
            
        except Exception as e:
            print(f"Error sending message to peer {peer_info}: {e}")
            return None
        finally:
            try:
                writer.close()
                await writer.wait_closed()
            except:
                pass