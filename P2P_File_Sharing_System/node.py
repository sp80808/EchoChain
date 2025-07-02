import asyncio
from networking import Networking
from dht import DHT
from file_manager import FileManager
from api import LocalAPI
import blockchain

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
        """
        # TODO: Implement message routing logic here, using DHT, FileManager, etc.
        pass

    # Add more orchestrator methods as needed for coordination 