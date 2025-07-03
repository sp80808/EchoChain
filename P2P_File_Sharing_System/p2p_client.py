import asyncio
import json
import os


class P2PClient:
    def __init__(self, node_host, local_api_port):
        self.node_host = node_host
        self.local_api_port = local_api_port

    async def _send_local_command(self, command_type, payload):
        try:
            reader, writer = await asyncio.open_connection(
                self.node_host, self.local_api_port
            )
            message = {"type": command_type, "payload": payload}
            writer.write(json.dumps(message).encode())
            await writer.drain()

            response_data = await reader.read(4096)
            response = json.loads(response_data.decode())

            writer.close()
            await writer.wait_closed()
            return response
        except Exception as e:
            print(
                f"Error sending local command to {self.node_host}:{self.local_api_port}: {e}"
            )
            return {"status": "error", "message": str(e)}

    async def add_file_and_announce(self, filepath):
        print(f"Client: Requesting to add file {filepath} to P2P system...")
        response = await self._send_local_command(
            "local_add_file", {"filepath": filepath}
        )
        if response.get("status") == "success":
            file_hash = response.get("file_hash")
            print(f"Client: File added with hash {file_hash}. Announcing content...")
            announce_response = await self._send_local_command(
                "local_announce_content", {"content_hash": file_hash}
            )
            if announce_response.get("status") == "success":
                print(f"Client: Content {file_hash} announced successfully.")
                return file_hash
            else:
                print(
                    f"Client: Failed to announce content: {announce_response.get('message')}"
                )
                return None
        else:
            print(f"Client: Failed to add file: {response.get('message')}")
            return None

    async def discover_content_peers(self, content_hash):
        print(f"Client: Requesting peers for content hash {content_hash}...")
        response = await self._send_local_command(
            "local_request_content_info", {"content_hash": content_hash}
        )
        if response.get("status") == "success":
            peers = response.get("peers", [])
            print(f"Client: Discovered peers for {content_hash}: {peers}")
            return peers
        else:
            print(
                f"Client: Failed to discover content peers: {response.get('message')}"
            )
            return []

    async def request_file_download(self, content_hash):
        print(f"Client: Requesting download for content hash {content_hash}...")
        response = await self._send_local_command(
            "local_request_file", {"content_hash": content_hash}
        )
        if response.get("status") == "success":
            print(
                f"Client: File download initiated for {content_hash}. (Check node logs for progress)"
            )
            return True
        else:
            print(
                f"Client: Failed to initiate file download: {response.get('message')}"
            )
            return False


# Example usage (for testing the client library independently or with a running node)
async def main():
    # Assuming a P2PNode is running on 127.0.0.1:8000 with local API on 127.0.0.1:8002
    client = P2PClient("127.0.0.1", 8002)

    # 1. Add a dummy file and announce it
    dummy_file_path = "P2P_File_Sharing_System/client_dummy_upload.txt"
    # Ensure the directory exists
    os.makedirs(os.path.dirname(dummy_file_path), exist_ok=True)
    with open(dummy_file_path, "w") as f:
        f.write("This is a dummy file uploaded via the client library. " * 50)

    uploaded_hash = await client.add_file_and_announce(dummy_file_path)
    if uploaded_hash:
        print(
            f"\nClient: Successfully uploaded and announced file with hash: {uploaded_hash}"
        )
    else:
        print("\nClient: Failed to upload and announce file.")

    await asyncio.sleep(2)  # Give node time to process

    # 2. Discover content peers for a known hash (e.g., the one just uploaded)
    if uploaded_hash:
        peers_for_content = await client.discover_content_peers(uploaded_hash)
        print(f"\nClient: Peers holding {uploaded_hash}: {peers_for_content}")

    await asyncio.sleep(2)

    # 3. Request a file download (this will trigger the placeholder in the node)
    if uploaded_hash:
        download_initiated = await client.request_file_download(uploaded_hash)
        if download_initiated:
            print(f"\nClient: Download request for {uploaded_hash} sent.")
        else:
            print(f"\nClient: Download request for {uploaded_hash} failed.")

    print("\nClient operations complete.")


if __name__ == "__main__":
    asyncio.run(main())
