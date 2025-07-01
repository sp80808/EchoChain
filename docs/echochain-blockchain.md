# EchoChain Blockchain Documentation

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
- **Purpose:** Register, approve, and track music samples on-chain.
- **Key Storage:**
  - `Samples`: Maps sample IDs to metadata (owner, IPFS CID, status, timestamp).
  - `NextSampleId`: Auto-incrementing sample ID.
- **Extrinsics:**
  - `register_sample(ipfs_cid, metadata_ipfs_cid)`: Register a new sample (trusted origin).
  - `update_sample_status(sample_id, new_status)`: Approve/reject samples (root/oracle).
- **Events:**
  - `SampleRegistered`, `SampleStatusUpdated`.

### b) Content Rewards Pallet
- **Purpose:** Distribute ECHO tokens to users with >= N approved samples each period.
- **Key Storage:**
  - `LastRewardBlock`: Last reward distribution block.
- **Extrinsics:**
  - `distribute_rewards(eligible_users)`: Distribute rewards (root/off-chain worker).
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
- **Purpose:** Reward users for seeding files and bandwidth.
- **Key Storage:**
  - `Reports`: User seeding reports.
  - `LastRewardBlock`: Last network reward distribution.
- **Extrinsics:**
  - `submit_report(bytes_uploaded, bytes_downloaded)`: Users report seeding.
  - `distribute_network_rewards()`: Distribute rewards (root/off-chain worker).
- **Events:**
  - `ReportSubmitted`, `NetworkRewardDistributed`.

---

## 3. Integration Points
- **macOS App:** Uses JSON-RPC for wallet, balance, transaction, and sample registration.
- **Backend API:** Registers samples, fetches metadata, and reports network contributions.
- **P2P System:** Reports seeding activity for network rewards.

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