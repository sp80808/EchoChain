# Acurast Modules: Structure & Mapping to EchoChain

## 1. Overview
This document summarizes the relevant modules from [Acurast/acurast-substrate](https://github.com/Acurast/acurast-substrate), their structure, and how they will be mapped and integrated into the EchoChain blockchain.

---

## 2. Acurast Modules: Structure & Purpose

### a. Marketplace Pallet (`acurast-marketplace`)
- **Purpose:**
  - Enables decentralized job posting, bidding, and fulfillment for off-chain compute tasks.
  - Handles job lifecycle: creation, assignment, completion, and settlement.
- **Key Features:**
  - Job posting and metadata
  - Bidding/assignment logic
  - Result submission and validation
  - Payment/reward distribution
- **Dependencies:** FRAME, balances, session, and custom types.

### b. Compute/Job Pallet (`acurast-protocol`)
- **Purpose:**
  - Manages distributed compute task execution and coordination.
  - Tracks job status, worker assignment, and result reporting.
- **Key Features:**
  - Task registration and scheduling
  - Worker registration and assignment
  - Result reporting and verification
- **Dependencies:** FRAME, balances, offchain workers, and custom types.

### c. Crypto Primitives (e.g., `p256-crypto`)
- **Purpose:**
  - Provides cryptographic primitives (e.g., p256 signatures) for secure compute and protocol operations.
- **Key Features:**
  - Signature verification
  - Key management
- **Dependencies:** Rust crypto crates, Substrate primitives.

### d. Node/Runtime Patterns
- **Purpose:**
  - Modern Substrate node and runtime configuration, chain spec management, and best practices.
- **Key Features:**
  - Modular runtime composition
  - Up-to-date chain spec and node setup
  - Integration with benchmarking, offchain workers, and RPCs

---

## 3. Mapping to EchoChain Architecture

| Acurast Module         | EchoChain Target Area         | Integration/Action                                   |
|-----------------------|------------------------------|-----------------------------------------------------|
| Marketplace Pallet    | Compute Marketplace Pallet   | Fork/adapt for music sample processing jobs, integrate with PoC and P2P for rewards and validation |
| Compute/Job Pallet    | Distributed Compute Pallet   | Fork/adapt for distributed audio analysis, validation, and future compute features              |
| Crypto Primitives     | Security Layer (optional)    | Integrate if advanced cryptography is needed for compute validation or secure offchain tasks    |
| Node/Runtime Patterns | Node/Runtime Infrastructure  | Update EchoChain node/runtime for maintainability, extensibility, and best practices           |

---

## 4. Integration Plan Alignment
- **Refactor** all forked modules to fit EchoChain's domain (naming, types, events, reward logic).
- **Integrate** with existing PoC, P2P, and governance pallets for seamless reward and parameter management.
- **Expose** necessary runtime APIs for compute/marketplace operations.
- **Document** all changes and maintain compatibility for future upstream updates.

---

## 5. References
- [Acurast/acurast-substrate](https://github.com/Acurast/acurast-substrate)
- [EchoChain INTEGRATION_PLAN.md](./INTEGRATION_PLAN.md) 