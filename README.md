# EchoChain: Decentralized Music Sample Marketplace

EchoChain is a modular, decentralized platform for music sample sharing, computation, and royalty distribution. It combines a custom Substrate-based blockchain, a peer-to-peer file sharing system, a native macOS app, a React-based GUI, and robust backend services to create a secure, transparent, and extensible ecosystem for creators, processors, and storage providers.

---

## 🚀 Project Overview
- **Purpose:** Enable decentralized registration, sharing, and monetization of music samples and data assets.
- **Key Features:**
  - Custom blockchain with modular pallets for marketplace, compute, royalty, and P2P integration
  - Peer-to-peer file sharing with DHT and BitTorrent-like protocol
  - Native macOS app for wallet, sample management, and P2P
  - React GUI for user-friendly blockchain and marketplace interaction
  - Automated, transparent royalty and reward distribution

---

## 🛠️ Tech Stack
- **Blockchain:** Substrate (Rust), custom pallets, Polkadot SDK, ink! smart contracts
- **Backend:** Node.js (TypeScript), @polkadot/api
- **Frontend:** React.js, notistack, axios
- **P2P System:** Python (asyncio), DHT, file chunking
- **macOS App:** Swift, SwiftUI, Substrate.swift
- **Containerization:** Docker, docker-compose

See the full [Tech Stack](cline_docs/techStack.md) for details.

---

## 🗺️ Roadmap
- Develop and integrate core blockchain pallets
- Build marketplace, compute, royalty, and P2P modules
- Complete GUI and macOS app integration
- Achieve full end-to-end decentralized workflows
- See the full [Project Roadmap](cline_docs/projectRoadmap.md) for milestones and progress.

---

## 📚 Documentation Index
- [Comprehensive Documentation & Development Plan](docs/EchoChain_Documentation_and_Development_Plan.md)
- [Architecture Overview](docs/architecture.md)
- [Tech Stack](cline_docs/techStack.md)
- [Project Roadmap](cline_docs/projectRoadmap.md)
- [P2P File Sharing System](P2P_File_Sharing_System/README.md)
- [macOS Application](macOS_Application/README.md)
- [Backend API Services](Backend_API_Services/README.md)
- [Blockchain Node](Blockchain/README.md)
- [Blockchain GUI](Blockchain/Blockchain_GUI/README.md)
- [Docs Index](docs/README.md)

---

## 🏁 Getting Started
- See [docs/EchoChain_Documentation_and_Development_Plan.md](docs/EchoChain_Documentation_and_Development_Plan.md) for setup, build, and usage instructions for each component.
- Each subproject contains a dedicated README with detailed instructions.

---

## 🔗 Cross-References
- All major documentation files are cross-linked above. For additional docs, see [docs/README.md](docs/README.md).
- For contributing, see the [Development Plan](docs/EchoChain_Documentation_and_Development_Plan.md#contributing).

---

[Back to Main README](README.md)

## Overview
This repository contains the source code and documentation for EchoChain, a decentralized music sample marketplace. The project is divided into several main sub-directories:

*   **[Blockchain](./Blockchain/README.md)**: Custom EchoChain blockchain.
*   **[Backend API Services](./Backend_API_Services/README.md)**: Lightweight backend APIs for authentication, copyright, and integrations.
*   **[P2P File Sharing System](./P2P_File_Sharing_System/README.md)**: Decentralized peer-to-peer file sharing.
*   **[macOS Application](./macOS_Application/README.md)**: Native macOS app for wallet, sample browsing, and P2P client.
*   **[Landing Page](./LandingPage/README.md)**: Project landing page and macOS application UI.
*   **[Sample Browser App](./Sample%20Browser%20App/README.md)**: Core sample browsing and management application.

## Project Vision
EchoChain is a decentralized, community-driven music sample sharing platform. It provides a modern alternative to services like Splice, enabling music producers to discover, share, and download high-quality, royalty-free audio samples. The ecosystem is powered by a purpose-built blockchain and its native cryptocurrency, the "Echo Token" (ECHO). The core philosophy is to directly reward creators for their contributions and minimize centralized infrastructure costs through a peer-to-peer file-sharing model.

## Usage, Building, and Development

### How to Use EchoChain
- EchoChain enables music producers to browse, share, and download audio samples in a decentralized, community-driven environment.
- Users can interact with the platform via the macOS app, web sample browser, or API endpoints.

### Building and Running Components
- **Blockchain:** See [Blockchain/README.md](./Blockchain/README.md) for instructions on building and running the custom EchoChain node.
- **Backend API Services:** See [Backend_API_Services/README.md](./Backend_API_Services/README.md) for setup, environment configuration, and API usage.
- **P2P File Sharing System:** See [P2P_File_Sharing_System/README.md](./P2P_File_Sharing_System/README.md) for running the decentralized file sharing node.
- **macOS Application:** See [macOS_Application/README.md](./macOS_Application/README.md) for building and running the native app in Xcode.
- **Sample Browser App:** See [Sample Browser App/README.md](./Sample%20Browser%20App/README.md) for web-based sample browsing and management.

### Contributing
- Contributions are welcome! Please review the development plans and guidelines in each subproject's README.
- Submit issues or pull requests for bug fixes, features, or documentation improvements.

### Further Documentation
- For detailed technical documentation and development plans, see [EchoChain_Documentation_and_Development_Plan.md](./EchoChain_Documentation_and_Development_Plan.md).
- Each subproject contains its own README with component-specific instructions and details.
