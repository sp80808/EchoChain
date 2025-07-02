# Architecture Overview

> **See also:** [Echochain Blockchain Spec](./echochain-blockchain.md), [Main Documentation Index](./README.md)

# EchoChain System Architecture

## Overview

```mermaid
graph TD
    A[macOS App] -->|REST API| B(Backend Services)
    A -->|WebSockets| C(Blockchain Node)
    A -->|P2P Protocol| D(P2P Network)
    B -->|REST API| E[External Services]
    C -->|On-chain| F[Smart Contracts]
    C -->|On-chain| G[XCM Messaging]
    D -->|IPFS| H[Distributed Storage]
```

## Blockchain Component Architecture

```