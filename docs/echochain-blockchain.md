# EchoChain Blockchain Documentation

> **See also:** [Architecture Overview](./architecture.md), [Content Rewards Pallet](./content-rewards-pallet.md), [Network Rewards Pallet](./network-rewards-pallet.md), [Main Documentation Index](./README.md)

## Overview
EchoChain is a novel, gas-free blockchain designed for decentralized music sample sharing, contribution rewards, and seamless integration with P2P and backend services.

---

## 1. Architecture
- **Framework:** Built on Substrate/Polkadot SDK.
- **Consensus:** Configurable (Aura/Babe for PoA/PoS, can be customized).
- **Token:** ECHO (native, 12 decimals).
- **Node Types:** Full, light, and validator nodes supported.

---

## 2. Core Protocols & Pallets
### a) Sample Registry Pallet
- **Purpose:** Register, approve, and track music samples on-chain. Provides an interface for other pallets to query approved sample counts.
- **Key Storage:**
  - `Samples`: Maps sample IDs to metadata (owner, IPFS CID, status, timestamp).
  - `NextSampleId`: Auto-incrementing sample ID.
- **Extrinsics:**
  - `register_sample(ipfs_cid, metadata_ipfs_cid)`: Register a new sample (trusted origin).
  - `update_sample_status(sample_id, new_status)`: Approve/reject samples (root/oracle).
- **Events:**
  - `SampleRegistered`, `SampleStatusUpdated`.
- **Interfaces:**
  - `SampleInterface`: Provides `get_approved_sample_count` for querying approved samples by user.

### b) Content Rewards Pallet
- **Purpose:** Distribute ECHO tokens to users with >= N approved samples each period. Integrates with the Sample Registry Pallet to determine eligibility.
- **Key Storage:**
  - `LastRewardBlock`: Last reward distribution block.
- **Extrinsics:**
  - `distribute_rewards()`: Distribute rewards (root/off-chain worker).
- **Events:**
  - `RewardDistributed`.
- **Automation:** Off-chain worker triggers distribution at set intervals.

### c) Proof-of-Contribution Pallet
- **Purpose:** Track and reward both content and network contributions.
- **Key Storage:**
  - `ContentContributions`, `NetworkContributions`, `TotalUnclaimedRewards`.
- **Extrinsics:**
  - `report_network_contribution(storage_bytes, bandwidth_bytes)`: Users report P2P activity.
- **Events:**
  - `ContentRewardsDistributed`, `NetworkRewardsDistributed`, `NetworkContributionReported`.

### d) Network Rewards Pallet
- **Purpose:** Reward users for seeding files and bandwidth. Provides an interface for other pallets to submit seeding reports.
- **Key Storage:**
  - `Reports`: User seeding reports.
  - `LastRewardBlock`: Last network reward distribution.
- **Extrinsics:**
  - `submit_report(bytes_uploaded, bytes_downloaded)`: Users report seeding.
  - `distribute_network_rewards()`: Distribute rewards (root/off-chain worker).
- **Events:**
  - `ReportSubmitted`, `NetworkRewardDistributed`.
- **Interfaces:**
  - `NetworkRewardsInterface`: Provides `submit_report` for other pallets to report network activity.

### e) P2P Integration Pallet
- **Purpose:** Facilitate integration with the P2P file sharing system, including reporting activity and triggering compute jobs.
- **Extrinsics:**
  - `register_p2p_node()`: Registers a P2P node.
  - `report_p2p_activity(bytes_uploaded, bytes_downloaded)`: Reports P2P seeding activity to the Network Rewards Pallet.
  - `trigger_compute_job(job_id, task_id, job_details)`: Triggers a compute job on the Echochain Compute and Marketplace Pallets.
- **Events:**
  - `P2PNodeRegistered`, `P2PActivityReported`, `ComputeJobTriggered`.

### f) Echochain Compute Pallet
- **Purpose:** Manages distributed compute tasks, such as audio analysis or sample validation, leveraging Acurast integration.
- **Extrinsics:**
  - `create_compute_task(task_id)`: Creates a new compute task.
- **Events:**
  - `ComputeTaskCreated`.
- **Interfaces:**
  - `ComputeInterface`: Provides `create_task` for other pallets to create compute tasks.

### g) Echochain Marketplace Pallet
- **Purpose:** Enables decentralized job posting and fulfillment for compute tasks related to music samples, leveraging Acurast integration.
- **Extrinsics:**
  - `post_compute_job(job_id, job_details)`: Posts a new compute job to the marketplace.
- **Events:**
  - `ComputeJobPosted`.
- **Interfaces:**
  - `MarketplaceInterface`: Provides `post_job` for other pallets to post compute jobs.

---

## 3. Integration Points
- **macOS App:** Uses JSON-RPC for wallet, balance, transaction, and sample registration.
- **Backend API:** Registers samples, fetches metadata, and reports network contributions.
- **P2P System:** Reports seeding activity for network rewards via the P2P Integration Pallet, which can also trigger compute jobs.
- **Acurast Integration:** The Echochain Compute and Echochain Marketplace pallets are designed to integrate with Acurast for decentralized compute capabilities.

---

## 4. Security & Gas-Free Design
- **Gas-Free:** No transaction fees for core actions (custom runtime logic, no extrinsic fees for key operations).
- **Key Management:** Private keys stored in Secure Enclave/Keychain (macOS), never exposed.
- **Access Control:** Only trusted origins can register/approve samples; rewards are automated.

---

## 5. Deployment Guide
1. **Build the Node:**
   - `cd Blockchain/echochain-node`
   - `cargo build --release`
2. **Run a Local Node:**
   - `./target/release/echochain-node --dev`
3. **Launch a Testnet:**
   - Configure bootnodes and genesis in `chain_spec.rs`.
   - Deploy multiple nodes (cloud or local).
   - Use Prometheus/Grafana for monitoring.
4. **Interact:**
   - Use Polkadot.js Apps, custom UI, or JSON-RPC clients.

---

## 6. References
- See `docs/content-rewards-pallet.md`, `docs/network-rewards-pallet.md`, and `Blockchain/README.md` for more details. 

## Related Documentation

*   [Main EchoChain Project README](../README.md)
*   [Architecture Overview](./architecture.md)
*   [EchoChain Documentation and Development Plan](./EchoChain_Documentation_and_Development_Plan.md) 

## Related Documentation

*   [Main EchoChain Project README](../README.md)
*   [Architecture Overview](./architecture.md)
*   [EchoChain Documentation and Development Plan](./EchoChain_Documentation_and_Development_Plan.md)