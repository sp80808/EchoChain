# CODEBASE_INDEX.md

This file provides a high-level index and mapping of the EchoChain codebase for quick reference.

## Main Components

- **Blockchain/**
  - Custom Layer 1 blockchain (Substrate, Rust)
  - Contains: echochain-node, pallets, runtime, scripts
- **Backend_API_Services/**
  - Lightweight backend APIs (Node.js/Express, TypeScript)
  - Contains: src, LandingPage
- **P2P_File_Sharing_System/**
  - Decentralized file sharing (Python)
  - Contains: p2p clients, audio analysis service
- **macOS_Application/**
  - Native macOS app (Swift/SwiftUI)
  - Contains: EchoChainApp
- **LandingPage/**
  - Web UI (React, Tailwind CSS, Electron)
  - Contains: src, project
- **Sample Browser App/**
  - Sample browsing/management (React/Electron)
  - Contains: src, server
- **docs/**
  - Documentation and plans

## Key Documentation

- `/README.md`: Project overview
- `/docs/EchoChain_Documentation_and_Development_Plan.md`: Master plan
- `/Blockchain/README.md`: Blockchain plan
- `/Backend_API_Services/README.md`: Backend API plan
- `/P2P_File_Sharing_System/README.md`: P2P plan
- `/macOS_Application/README.md`: macOS app plan
- `/LandingPage/README.md`: UI/UX plan
- `/Sample Browser App/README.md`: Sample browser plan

## Integration Points

- Blockchain <-> Backend API: Tokenomics, user data
- Blockchain <-> P2P: Content metadata, ownership
- macOS App <-> All: Wallet, sample management, P2P client
- Landing Page <-> Backend API: User auth, sample data
- Sample Browser <-> Backend, Blockchain, P2P

## Update Policy

Update this index as the codebase evolves. For details, see the master documentation and each component's README.
