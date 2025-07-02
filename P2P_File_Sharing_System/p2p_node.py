# DEPRECATED: Logic moved to node.py and modular files (networking.py, dht.py, file_manager.py, api.py, blockchain.py)
# This file is retained for backward compatibility and migration reference only.
# Use node.py for the main entry point.

from node import EchoChainNode
import asyncio

async def main():
    node = EchoChainNode('127.0.0.1', 8000, 8002)
    await node.start()

if __name__ == "__main__":
    asyncio.run(main())