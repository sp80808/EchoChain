# EchoChain P2P File Sharing System

## Overview
This component implements the decentralized peer-to-peer file sharing system for the EchoChain platform, enabling distributed storage and transfer of music samples.

## Modular Architecture (2025 Refactor)
- `networking.py`: Peer discovery, connection handling, message passing
- `dht.py`: Distributed Hash Table for content/peer management
- `file_manager.py`: File chunking, hashing, storage
- `api.py`: Local API for integration with macOS app and other clients
- `blockchain.py`: Blockchain hooks for registering/verifying file metadata
- `security.py`: (Planned) Peer authentication, encryption, advanced integrity checks

## Local API
- The local API allows clients (e.g., macOS app, Node.js client) to add files, announce content, request downloads, and more.
- See `api.py` for command structure and examples.

## Blockchain Integration
- File hashes and metadata can be registered and verified on-chain via `blockchain.py`.
- Future: Use blockchain events to trigger P2P actions and reward distribution.

## Purpose
- Decentralized storage and distribution of music samples
- Reduces reliance on central servers

## Architectural Design
- DHT (Distributed Hash Table) for content discovery
- BitTorrent-like protocol for file transfer
- Python-based implementation

## Core Technologies
- Python 3.x
- Networking libraries (asyncio, socket, etc.)
- Hashing algorithms for content addressing

## Dependencies
- Internal: EchoChain Blockchain for metadata and ownership
- External: None (self-contained P2P network)

## API Specifications
- Local API for macOS App to request/provide files
- P2P protocol specifications

## Data Models
- FileChunk, Peer, Metadata

## Security & Compliance
- Data integrity and authenticity checks
- Protection against malicious peers
- No sensitive data or secrets are stored in the repository
- `.env` and virtual environments are ignored (see .gitignore)

## Testing
- Unit tests for hashing and chunking
- Network simulation for peer discovery and file transfer

## .gitignore Rationale
- `__pycache__/`, `*.pyc`, `.venv/`, `venv/`, `.env` are ignored for security and compliance

## Getting Started
1. Create and activate a virtual environment: `python -m venv venv && source venv/bin/activate`
2. Install dependencies: `pip install -r requirements.txt`
3. **Run the node:**
   - Main entry point: `python node.py`
   - (Legacy: `python p2p_node.py` is deprecated and will be removed in future releases)

## API Endpoints (Local API)
The local API (see `api.py`) supports the following commands (JSON-over-TCP):
- `local_add_file`: Add a file to the node
- `local_announce_content`: Announce a file/content hash to the network
- `local_request_content_info`: Get peers for a content hash or list all available content
- `local_request_file`: Request a file download from the network
- `local_list_content`: List all content hashes in the DHT
- `local_register_file_on_chain`: Register file metadata on the blockchain (stub)
- `local_verify_file_on_chain`: Verify file metadata on the blockchain (stub)

See `api.py` and `P2PClient_Usage_Examples.md` for usage from Python, Node.js, Go, Bash, and C# clients.

## Integration Flows
- **Add and announce a file:**
  1. `local_add_file` → 2. `local_announce_content`
- **Discover peers for content:**
  - `local_request_content_info`
- **Request file download:**
  - `local_request_file`
- **Blockchain registration/verification:**
  - `local_register_file_on_chain`, `local_verify_file_on_chain`

## Migration Notice
- `p2p_node.py` is **deprecated**. All logic has been modularized. Use `node.py` as the main entry point.

## Development Plan
- See main project documentation for detailed milestones and phases. 

## Related Documentation

*   [Main EchoChain Project README](../../README.md)
*   [EchoChain Documentation and Development Plan](../../docs/EchoChain_Documentation_and_Development_Plan.md)
*   [Architecture Overview](../../docs/architecture.md) 

## Related Documentation

*   [Main EchoChain Project README](../../README.md)
*   [EchoChain Documentation and Development Plan](../../docs/EchoChain_Documentation_and_Development_Plan.md)
*   [Architecture Overview](../../docs/architecture.md) 