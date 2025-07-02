import asyncio
import json

class LocalAPI:
    def __init__(self, node, api_port):
        self.node = node
        self.api_port = api_port

    async def start(self):
        server = await asyncio.start_server(self.handle_connection, '127.0.0.1', self.api_port)
        print(f"Local API server started on 127.0.0.1:{self.api_port}")
        async with server:
            await server.serve_forever()

    async def handle_connection(self, reader, writer):
        addr = writer.get_extra_info('peername')
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
        # Stub: expand with actual command handling
        cmd_type = message.get('type')
        payload = message.get('payload', {})
        if cmd_type == 'local_add_file':
            file_hash = self.node.file_manager.add_file(payload.get('filepath'))
            if file_hash:
                return {'status': 'success', 'file_hash': file_hash}
            else:
                return {'status': 'error', 'message': 'File not found'}
        # Add more commands as needed
        return {'status': 'error', 'message': 'Unknown command'} 