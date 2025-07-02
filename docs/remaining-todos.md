# Remaining TODOs and FIXMEs

This document lists the remaining TODOs and FIXMEs identified in the Echochain codebase that are not being addressed in the current development phase. These items are categorized by their nature and location, with a brief explanation for why they are not being tackled at this time.

---

## 1. External Dependencies (Polkadot SDK & Acurast Substrate)

Many TODOs and FIXMEs are found within the `polkadot-sdk` and `acurast-substrate` directories. These are external projects, and modifying their code directly is generally not advisable as it would break compatibility with upstream versions and introduce maintenance overhead. These comments often represent: 

*   **Upstream Issues:** Bugs or feature gaps identified by the original developers of the SDKs.
*   **Future Enhancements:** Planned features or optimizations by the SDK developers.
*   **Design Considerations:** Notes on architectural choices or potential improvements within the SDKs themselves.

**Action:** These items are best addressed by contributing directly to the respective upstream projects or by waiting for official updates that resolve them. They are not within the direct control or immediate scope of the Echochain project's internal development.

**Examples:**

*   `Blockchain/acurast-substrate/Cargo.toml: # TODO: remove once tokenomics are ready`
*   `Blockchain/acurast-substrate/frame/support/procedural/src/pallet/expand/documentation.rs: // TODO: Use [drain_filter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter) when it is stable.`
*   `Blockchain/polkadot-sdk/.github/workflows/zombienet_polkadot.yml: # TODO: Disabled, occasionally (1 on ~50-70 runs) fails`
*   `Blockchain/polkadot-sdk/substrate/client/rpc/src/chain/mod.rs: // FIXME <2329>: Database seems to limit the block number to u32 for no reason`

---

## 2. Benchmarking Implementations

Several TODOs relate to implementing or refining benchmarks for Substrate pallets. While crucial for accurate weight calculation and network performance, setting up a robust and reliable benchmarking environment for Substrate can be complex, especially when dealing with intricate dependency trees and external SDKs. Previous attempts to resolve these issues highlighted significant challenges in configuring the build environment.

**Action:** Full implementation of these benchmarks is deferred. It requires dedicated effort to stabilize the benchmarking setup, potentially involving a deeper understanding of the `frame-benchmarking` intricacies and its interaction with the `polkadot-sdk`'s build system.

**Examples:**

*   `Blockchain/echochain-node/pallets/echochain-compute/src/traits.rs: // TODO: Replace with auto-generated weights from real benchmarks. See issue #124.`
*   `Blockchain/echochain-node/pallets/network-rewards/src/benchmarking.rs: // TODO: Implement real benchmarks for all extrinsics. See issue #123.`

---

## 3. Complex Pallet Logic & Architectural Enhancements

Some TODOs indicate areas where the current pallet logic is a placeholder or requires significant architectural changes for full implementation. These are not simple fixes and would involve substantial design and development work.

**Action:** These items are considered future development phases. They require detailed design specifications, potential changes to the blockchain's core logic, and thorough testing.

**Examples:**

*   `Blockchain/echochain-node/pallets/echochain-marketplace/src/payments.rs: // TODO refunded amount is collected on hyperdrive_pallet_account but not yet refunded to proxy chain`
*   `Blockchain/echochain-node/node/src/service.rs: // FIXME #1578 make this available through chainspec`

---

## 4. Documentation & Minor Code Refinements

These are comments that suggest improvements to documentation, minor code refactorings, or considerations that do not impact the current functionality or stability of the system.

**Action:** These can be addressed in future maintenance cycles or as part of broader refactoring efforts. They are not critical for the current operational state.

**Examples:**

*   `Blockchain/build-open-source.md: | Validator Dashboards | [Polkadot Telemetry](https://github.com/paritytech/substrate-telemetry) ⚪, ...` (This is a list of external dashboards, not a TODO for implementation)
*   `macOS_Application/Development_Plan.md: - [ ] Implement send/receive ECHO token functionality. (In Progress: `sendTransaction` method updated with detailed Substrate extrinsic TODOs)` (This is a development plan item, not a code TODO)
*   `macOS_Application/EchoChainApp/P2PClient.swift: // TODO: For production, consider a more robust JSON-RPC client or a dedicated P2P communication protocol.`
*   `macOS_Application/EchoChainApp/SampleUploadView.swift: // TODO: Properly handle security-scoped bookmarks for persistent access if needed.`

---

This document provides clarity on the remaining development tasks and their current status within the Echochain project. Future efforts can prioritize these items based on project goals and resource availability.
