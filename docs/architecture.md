# EchoChain System Architecture

## Overview

```mermaid
graph TD
    A[macOS App] -->|REST API| B(Backend Services)
    A -->|WebSockets| C(Blockchain Node)
    A -->|P2P Protocol| D(P2P Network)
    B -->|REST API| E[External Services]
    C -->|On-chain| F[Smart Contracts]
    D -->|IPFS| G[Distributed Storage]
```

## Blockchain Component Architecture

```mermaid
graph LR
    subgraph Blockchain Node
        A[Runtime] --> B[Proof of Contribution]
        A --> C[Sample Registry]
        A --> D[Governance]
        A --> E[Balances]
        B --> F[Reward Distribution]
        C --> G[Content Metadata]
    end
    
    H[macOS App] -->|RPC| A
    I[Other Nodes] -->|P2P| A
```

## Data Flow

```mermaid
sequenceDiagram
    participant User
    participant App
    participant Backend
    participant Blockchain
    participant P2P
    
    User->>App: Upload Sample
    App->>Backend: Copyright Check
    Backend-->>App: Check Result
    App->>Blockchain: Register Metadata
    Blockchain-->>App: Transaction Receipt
    App->>P2P: Distribute Content
    P2P-->>App: Content Hash
    App->>Blockchain: Store Content Hash
```

## Key Components

### 1. macOS Application
- **Responsibilities**:
  - User interface and experience
  - Wallet management
  - Sample browsing/uploading
  - P2P client integration

### 2. Backend Services
- **Responsibilities**:
  - User authentication
  - Copyright verification
  - API integrations
  - Caching layer

### 3. Blockchain Node
- **Responsibilities**:
  - Content registration
  - Proof of Contribution consensus
  - Reward distribution
  - Governance

### 4. P2P Network
- **Responsibilities**:
  - Distributed content storage
  - Content discovery
  - Efficient data transfer

## Interaction Patterns

1. **Sample Upload Flow**:
   - Content checked via Backend
   - Metadata recorded on Blockchain
   - Content distributed via P2P
   - Content hash stored on-chain

2. **Reward Distribution**:
   - Contributions tracked on-chain
   - Periodic reward calculations
   - Automated payouts

3. **Governance**:
   - On-chain proposals
   - Voting mechanisms
   - Parameter updates