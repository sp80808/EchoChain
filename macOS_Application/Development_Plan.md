# EchoChain macOS Application - Development Plan

This document outlines the key development areas and tasks required to complete the EchoChain macOS application, building upon the existing structure and `README.md`.

## 1. Core Feature Implementation

### 1.1 Wallet Management (`WalletView.swift`, `SecureStorage.swift`)
- [ ] Implement secure wallet creation (new seed phrase generation).
- [x] Implement wallet import (from seed phrase or private key).
- [ ] Display current ECHO token balance and transaction history.
- [x] Integrate with `BlockchainClient.swift` for on-chain balance queries. (Completed: Updated `fetchBalance` with more accurate Substrate RPC method and TODOs for SCALE decoding)
- [x] Implement send/receive ECHO token functionality. (In Progress: `sendTransaction` method updated with detailed Substrate extrinsic TODOs)
- [x] Enhance `SecureStorage.swift` for robust key management (e.g., using Keychain services).

### 1.2 Sample Browsing (`SampleBrowserView.swift`)
- [ ] Implement fetching and displaying a list of available audio samples from the backend API. (In Progress: Integrating with Backend API for richer metadata)
- [ ] Integrate search and filtering capabilities (genre, BPM, key, etc.).
- [x] Implement audio playback functionality for samples. (Refined TODO for streaming large files)
- [ ] Display detailed sample metadata (creator, description, price, license).
- [ ] Implement sample purchase flow, interacting with `BlockchainClient.swift`.

### 1.3 Sample Uploading (`SampleUploadView.swift`)
- [x] Implement UI for selecting and uploading audio files.
- [ ] Integrate with backend API for metadata submission and file processing.
- [ ] Implement progress tracking for uploads.
- [ ] Add validation for audio file types and sizes.
- [ ] Integrate with `P2PClient.swift` for P2P distribution of uploaded assets.

### 1.4 P2P Client Integration (`P2PClient.swift`)
- [ ] Establish connection to the P2P network.
- [ ] Implement seeding functionality for owned or downloaded samples.
- [ ] Implement fetching/downloading samples from the P2P network.
- [ ] Handle network connectivity changes and re-connections.
- [ ] Integrate with the main app for displaying P2P status.

## 2. Blockchain Integration (`BlockchainClient.swift`)
- [x] Implement connection to the EchoChain blockchain node. (Docker setup and basic Swift client connectivity check added)
- [ ] Develop methods for interacting with smart contracts (e.g., token transfers, sample listing, purchase).
- [ ] Handle blockchain transaction signing and submission.
- [ ] Implement event listeners for on-chain activities relevant to the user (e.g., new rewards, purchase confirmations).
- [ ] Error handling and retry mechanisms for blockchain interactions.

## 3. UI/UX Enhancements (`ContentView.swift`)
- [ ] Refine overall application layout and navigation.
- [ ] Implement responsive design for various macOS window sizes.
- [ ] Add visual feedback for user actions (loading states, success/error messages).
- [ ] Implement user settings and preferences.
- [ ] Ensure consistent styling and adherence to Apple's Human Interface Guidelines.

## 4. Testing
- [ ] **Unit Tests:** Write comprehensive unit tests for all business logic (e.g., wallet operations, data parsing, P2P logic).
- [ ] **UI Tests:** Develop UI tests to ensure critical user flows are functional and stable.
- [ ] **Integration Tests:** Create tests for interactions between the app, blockchain client, P2P client, and backend API.
- [ ] **Performance Testing:** Identify and optimize performance bottlenecks.

## 5. Build & Deployment
- [ ] Configure Xcode project for release builds.
- [ ] Implement code signing and notarization for macOS distribution.
- [ ] Set up continuous integration (CI) for automated testing and building.
- [ ] Explore options for automated deployment (e.g., GitHub Actions).

## 6. Documentation & Code Quality
- [ ] Add inline documentation (DocC comments) for all public APIs and complex logic.
- [ ] Ensure code adheres to Swift style guides and best practices.
- [ ] Review and refactor existing code for clarity, efficiency, and maintainability.

## Next Steps (Immediate)
1.  Review `BlockchainClient.swift` and `P2PClient.swift` to understand current integration points.
2.  Begin implementing basic wallet functionality in `WalletView.swift`.
3.  Set up a local development environment for the EchoChain blockchain and backend API to facilitate testing.
