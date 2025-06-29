# EchoChain Project Documentation

## Overview
This repository contains the source code and documentation for EchoChain, a decentralized music sample marketplace. The project is divided into several main sub-directories:

*   **[Blockchain](./Blockchain/README.md)**: Custom EchoChain blockchain.
*   **[Backend API Services](./Backend_API_Services/README.md)**: Lightweight backend APIs for authentication, copyright, and integrations.
*   **[P2P File Sharing System](./P2P_File_Sharing_System/README.md)**: Decentralized peer-to-peer file sharing.
*   **[macOS Application](./macOS_Application/README.md)**: Native macOS app for wallet, sample browsing, and P2P client.
*   **[Landing Page](./Landing%20Page/README.md)**: Project landing page and macOS application UI.
*   **[Sample Browser App](./Sample%20Browser%20App/README.md)**: Core sample browsing and management application.

## Project Vision
EchoChain is a decentralized, community-driven music sample sharing platform. It provides a modern alternative to services like Splice, enabling music producers to discover, share, and download high-quality, royalty-free audio samples. The ecosystem is powered by a purpose-built blockchain and its native cryptocurrency, the "Echo Token" (ECHO). The core philosophy is to directly reward creators for their contributions and minimize centralized infrastructure costs through a peer-to-peer file-sharing model.

## Security, Compliance, and .gitignore Policy
- All secrets, credentials, and environment variables are managed via `.env` files, which are strictly ignored in all subprojects.
- Build artifacts, dependencies (e.g., `node_modules/`, `venv/`), IDE/editor files, and OS-specific files are ignored to prevent accidental leaks and repository bloat.
- Each subproject contains a tailored `.gitignore` and documentation explaining its rationale.
- No sensitive data or credentials are ever committed to the repository.

## Getting Started
Refer to the `README.md` file in each sub-directory for detailed information about the development plan, technical infrastructure, and key features of each component.

## Documentation & Development Plan
See `EchoChain_Documentation_and_Development_Plan.md` for comprehensive technical documentation and detailed development plans for all components.
