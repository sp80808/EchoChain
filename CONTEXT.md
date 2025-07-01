# EchoChain Project Context

This document provides context for AI assistants and automation tools working with the EchoChain project.

## Project Structure

```
echochain/
├── Blockchain/               # Custom blockchain implementation
├── Backend_API_Services/     # Lightweight backend services
├── P2P_File_Sharing_System/  # Decentralized file sharing
├── macOS_Application/        # Native client application
├── docs/                     # Documentation
├── schemas/                  # JSON schemas for data models
├── tests/                    # Test suites
└── pallets/                  # Substrate blockchain pallets
```

## Key Components

1. **Blockchain**
   - Entry: `Blockchain/echochain-node/src/main.rs`
   - Core pallets: `proof-of-contribution`, `sample-registry`
   - Smart contracts: `Blockchain/contracts/`

2. **Backend Services**
   - Entry: `Backend_API_Services/src/main.rs`
   - API docs: `Backend_API_Services/openapi.yaml`

3. **P2P System**
   - Entry: `P2P_File_Sharing_System/main.rs`
   - Protocol: `p2p/proto/file_transfer.proto`

4. **macOS Application**
   - Entry: `macOS_Application/EchoChainApp/App.swift`
   - Main components: `BlockchainClient`, `P2PClient`

## Development Workflow

1. **Running the stack**
   - Start blockchain nodes: `Blockchain/echochain-node/scripts/run-local-testnet.sh`
   - Start backend: `cd Backend_API_Services && cargo run`
   - Start P2P node: `cd P2P_File_Sharing_System && cargo run`

2. **Testing**
   - Blockchain: `cd Blockchain/echochain-node && cargo test`
   - Backend: `cd Backend_API_Services && cargo test`
   - BDD tests: `cd tests && cucumber`

3. **Code Generation**
   - Protobuf: `protoc --rust_out=. p2p/proto/file_transfer.proto`
   - OpenAPI: `openapi-generator generate -i Backend_API_Services/openapi.yaml -g rust-server`

## Important Conventions

1. **Code Style**
   - Rust: Follow standard rustfmt
   - Swift: Apple's Swift style guide
   - TypeScript: Airbnb style guide

2. **Documentation**
   - All Rust code must have doc comments
   - Public APIs must have OpenAPI/AsyncAPI specs
   - Architectural decisions go in `docs/adr/`

3. **Error Handling**
   - Use proper error types (no unwrap() in production code)
   - Log errors with context
   - Provide user-friendly error messages

## AI Assistant Tips

- Always check existing protocol files before making changes
- Validate changes against JSON schemas
- Update documentation when modifying APIs
- Run tests before submitting changes