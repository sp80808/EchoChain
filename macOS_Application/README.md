# EchoChain macOS Application

## Overview
This is the native macOS application for the EchoChain platform, providing the main user interface for wallet management, sample browsing, uploading, and P2P client integration.

## Purpose
- User-facing interface for interacting with EchoChain
- Integrates wallet, sample browser, uploader, and P2P client

## Architectural Design
- Swift/SwiftUI-based application
- MVC/MVVM architecture
- Integration with blockchain and P2P systems

## Core Technologies
- Swift, SwiftUI
- Core Data for local storage
- Networking libraries for API and blockchain interaction

## Dependencies
- Internal: EchoChain Blockchain SDK, P2P File Sharing System, Backend API
- External: None (all secrets managed locally)

## API Specifications
- Internal APIs for component interaction
- External APIs for blockchain and backend

## Data Models
- Wallet, SampleMetadata, UserPreferences

## Security & Compliance
- Local key storage security (Keychain)
- Secure communication with blockchain and backend
- `.env` and Xcode user/workspace files are ignored (see .gitignore)

## Testing
- Unit tests for business logic
- UI tests for user flows
- Integration tests for wallet and P2P functionality

## .gitignore Rationale
- Xcode user/workspace files, `.env`, and build artifacts are ignored for security and compliance

## Getting Started
1. Open `EchoChainApp.xcodeproj` in Xcode
2. Configure local environment variables if needed
3. Build and run the app

## Development Plan
- See main project documentation for detailed milestones and phases. 