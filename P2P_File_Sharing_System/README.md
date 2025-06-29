# EchoChain P2P File Sharing System

## Overview
This component implements the decentralized peer-to-peer file sharing system for the EchoChain platform, enabling distributed storage and transfer of music samples.

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
3. Run the node: `python p2p_node.py`

## Development Plan
- See main project documentation for detailed milestones and phases. 