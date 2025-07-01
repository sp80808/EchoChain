
# EchoChain Project Documentation

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
- For detailed technical documentation and development plans, see the [Comprehensive Documentation](#comprehensive-documentation) section below.
- Each subproject contains its own README with component-specific instructions and details.

## Comprehensive Documentation

This section provides a central hub for all documentation within the EchoChain project.

### Project READMEs

*   **[Main Project README](./README.md)**: Overview of the entire EchoChain project.
*   **[Backend API Services README](./Backend_API_Services/README.md)**: Setup, environment configuration, and API usage for backend services.
*   **[Blockchain README](./Blockchain/README.md)**: Instructions for building and running the custom EchoChain node.
    *   **[Echochain Node README](./Blockchain/echochain-node/README.md)**: Specific documentation for the Echochain node.
    *   **[Pallet Template README](./Blockchain/echochain-node/pallets/template/README.md)**: Documentation for the pallet template.
    *   **[Substrate Contracts Node README](./Blockchain/substrate-contracts-node/README.md)**: Documentation for the Substrate contracts node.
*   **[Landing Page README](./LandingPage/README.md)**: Information about the project landing page and macOS application UI.
*   **[macOS Application README](./macOS_Application/README.md)**: Building and running the native macOS app in Xcode.
    *   **[EchoChainApp README](./macOS_Application/EchoChainApp/README.md)**: Specific documentation for the EchoChain macOS application.
*   **[P2P File Sharing System README](./P2P_File_Sharing_System/README.md)**: Running the decentralized file sharing node.
*   **[Sample Browser App README](./Sample%20Browser%20App/README.md)**: Web-based sample browsing and management.

### General Documentation (docs directory)

*   **[Architecture Overview](./docs/architecture.md)**: High-level architectural design of the EchoChain project.
*   **[AsyncAPI Specification](./docs/asyncapi.yaml)**: Asynchronous API definitions.
*   **[Content Rewards Pallet Documentation](./docs/content-rewards-pallet.md)**: Details on the content rewards pallet.
*   **[EchoChain Documentation and Development Plan](./docs/EchoChain_Documentation_and_Development_Plan.md)**: Detailed technical documentation and future development plans.
*   **[Echochain Blockchain Overview](./docs/echochain-blockchain.md)**: General information about the Echochain blockchain.
*   **[Network Rewards Pallet Documentation](./docs/network-rewards-pallet.md)**: Details on the network rewards pallet.
*   **[Open Source Integration Plan](./docs/OpenSource_Integration_Plan.md)**: Plan for integrating open-source components.
*   **[Project Progress Log](./docs/PROGRESS.md)**: Log of project development progress.
*   **[Architecture Decision Records (ADR)](./docs/adr/0001-architecture-decisions.md)**:
    *   [ADR 0001 - Architecture Decisions](./docs/adr/0001-architecture-decisions.md)
    *   [ADR 0002 - Context Protocols](./docs/adr/0002-context-protocols.md)
*   **[Sample Metadata Schema](./docs/schemas/sample-metadata.schema.json)**: JSON schema for sample metadata.
