# Echochain Governance Decisions

## Overview
This document records the key governance decisions made for the Echochain blockchain, focusing on token economics and value stability mechanisms to ensure the platform's long-term sustainability without hyperinflation or deflation.

## Token Decimalization and Configuration
- **Decision Date**: February 7, 2025
- **Change**: Set the existential deposit for the Echochain token (ECHO) to 1 in the 'pallet-balances' configuration.
- **Rationale**: Implements a 0.00 decimalization policy, meaning the token has no fractional units. This simplifies token handling and aligns with user requirements for a non-decimal token system.
- **Impact**: Ensures that all transactions and balances are handled in whole units, reducing complexity in accounting and user interfaces.
- **Location**: Updated in 'Blockchain/echochain-node/runtime/src/lib.rs' under 'pallet-balances' configuration.

## Treasury Parameters for Value Stability
- **Decision Date**: February 7, 2025
- **Change**: Adjusted the treasury burn rate to 1% and set the spend period to 7 days.
- **Rationale**: 
  - A reduced burn rate prevents excessive token destruction which could lead to deflation, preserving token availability for network participants.
  - A 7-day spend period allows for controlled distribution of treasury funds, balancing token supply to avoid hyperinflation by spacing out minting or reward events.
- **Impact**: These settings aim to increase token value over time by carefully managing supply, ensuring stability for creators and users in the Echochain ecosystem.
- **Location**: Updated in 'Blockchain/echochain-node/runtime/src/lib.rs' under 'pallet-treasury' parameters.

## Governance Accessibility
- **Decision Date**: February 7, 2025
- **Change**: Set proposal bond values to 1 for both minimum and actual bond requirements.
- **Rationale**: Aligns with the non-decimal token system and ensures that governance participation remains accessible to all network participants, even those with minimal token holdings.
- **Impact**: Lowers the barrier to entry for proposing and voting on governance changes, fostering a more inclusive decision-making process.
- **Location**: Updated in 'Blockchain/echochain-node/runtime/src/lib.rs' under treasury parameter settings.

## Dynamic Royalty Distribution System
- **Decision Date**: February 7, 2025
- **Change**: Designed a royalty distribution system allocating 70% of tokens to original creators, 10% to network contributors, and 20% as a company fee directed to a liquidity pool.
- **Rationale**: Ensures fair compensation for creators while incentivizing network participation and maintaining liquidity for token trading and stability.
- **Impact**: Enhances creator incentivization, supports network maintenance, and provides liquidity for market operations, contributing to overall ecosystem health.
- **Location**: To be implemented in relevant pallets such as 'pallet-echochain-marketplace' for royalty distribution logic.

## NFT-Based Sample Licensing
- **Decision Date**: February 7, 2025
- **Change**: Implement NFT-based sample licensing with minimal on-chain weight for efficiency and scalability.
- **Rationale**: Using NFTs for licensing allows for unique, verifiable ownership and transfer of sample rights with reduced storage and computation overhead on the blockchain.
- **Impact**: Improves scalability by minimizing on-chain data, ensuring efficient transactions, and enabling a robust licensing framework for music samples.
- **Location**: To be integrated into 'pallet-sample-registry' or a dedicated NFT pallet for licensing management.

## Future Considerations
- **Dynamic Adjustments**: Governance mechanisms should be periodically reviewed to adjust burn rates or spend periods based on network activity and token velocity to maintain value stability.
- **Reward Calibration**: Integration with compute and marketplace pallets should ensure that token minting for rewards is balanced against network usage to prevent inflationary pressure.
- **Community Input**: Proposals for significant changes to token economics should be subject to community voting through the democracy and collective pallets to ensure alignment with stakeholder interests.

## References
- Integration Plan: 'Blockchain/INTEGRATION_PLAN.md'
- Runtime Configuration: 'Blockchain/echochain-node/runtime/src/lib.rs'
