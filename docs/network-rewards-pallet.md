# Network Rewards Pallet Documentation

## Overview
The Network Rewards Pallet distributes ECHO token rewards to users who contribute to the P2P network by seeding files. Rewards are distributed automatically at regular intervals (e.g., weekly or monthly) using an off-chain worker, based on reported upload volume.

## Purpose
- Incentivize users to seed and share files in the P2P network.
- Automate reward distribution based on user-reported and on-chain data.
- Ensure transparency and auditability via events and storage.

## Configuration
- `RewardPoolAccount`: Account holding the reward pool (treasury).
- `TotalRewardPerPeriod`: Total ECHO tokens distributed per period.
- `MinBytesUploaded`: Minimum bytes uploaded to qualify for rewards.
- `RewardPeriod`: Number of blocks between reward distributions.

## Storage
- `Reports`: Mapping of user to their seeding report for the current period.
- `LastRewardBlock`: The last block at which rewards were distributed.

## Extrinsics
- `submit_report(bytes_uploaded: u64, bytes_downloaded: u64)`
  - Called by users to report their seeding activity for the current period.
  - Only one report per user per period.
- `distribute_network_rewards()`
  - Callable by root or off-chain worker.
  - Distributes rewards proportionally to all eligible users for the current period.
  - Idempotent: can only be called once per period.

## Events
- `ReportSubmitted { user, uploaded, downloaded }`: Emitted when a user submits a report.
- `NetworkRewardDistributed { user, amount }`: Emitted for each user who receives a reward.

## Errors
- `AlreadyReportedThisPeriod`: User has already submitted a report for the current period.
- `NotEnoughContribution`: User's upload volume is below the minimum threshold.
- `RewardPoolEmpty`: The reward pool does not have enough funds (should be handled by treasury management).

## Off-Chain Worker Automation
- The off-chain worker runs at the configured interval (`RewardPeriod`).
- It triggers `distribute_network_rewards`, which distributes rewards and clears reports for the next period.
- Ensures rewards are distributed automatically and only once per period.

## Usage Examples
### User Report Submission
```sh
# User calls submit_report with their upload/download stats for the period
```

### Automated Distribution
- The off-chain worker will handle reward distribution at the configured interval.

## Anti-Cheat Notes
- Only one report per user per period is allowed.
- Minimum upload threshold enforced.
- For advanced anti-cheat, consider challenge-response or cross-verification mechanisms.

## Testing Guidelines
- Unit and integration tests should cover:
  - Report submission and validation
  - Reward calculation and distribution
  - Idempotency (no double rewards)
  - Event emission
  - Off-chain worker scheduling
  - Error handling (e.g., insufficient funds, already reported, not enough contribution)

## Security & Best Practices
- Only root or off-chain worker should be able to call `distribute_network_rewards`.
- Ensure the reward pool is funded and managed securely.
- Monitor events for auditing and analytics.

---
For more details, see the Rust doc comments in `pallets/network-rewards/src/lib.rs`. 

## Related Documentation

*   [Main EchoChain Project README](../README.md)
*   [EchoChain Documentation and Development Plan](./EchoChain_Documentation_and_Development_Plan.md)
*   [Architecture Overview](./architecture.md) 