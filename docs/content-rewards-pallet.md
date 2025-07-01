# Content Rewards Pallet Documentation

## Overview
The Content Rewards Pallet is responsible for distributing ECHO token rewards to users who have contributed a minimum number of approved samples. Rewards are distributed automatically at regular intervals (e.g., monthly) using an off-chain worker.

## Purpose
- Incentivize users to contribute high-quality, approved samples.
- Automate reward distribution based on on-chain data.
- Ensure transparency and auditability via events and storage.

## Configuration
- `MinApprovedSamples`: Minimum number of approved samples required to be eligible for rewards.
- `RewardAmount`: Amount of ECHO tokens distributed to each eligible user per period.
- `RewardPoolAccount`: Account holding the reward pool (treasury).
- `RewardPeriod`: Number of blocks between reward distributions (e.g., ~1 month).

## Storage
- `LastRewardBlock`: The last block at which rewards were distributed.

## Extrinsics
- `distribute_rewards(eligible_users: Vec<AccountId>)`
  - Callable by root or off-chain worker.
  - Distributes rewards to all eligible users for the current period.
  - Idempotent: can only be called once per period.

## Events
- `RewardDistributed { user, amount }`: Emitted for each user who receives a reward.

## Errors
- `AlreadyRewardedThisPeriod`: Rewards have already been distributed for the current period.
- `RewardPoolEmpty`: The reward pool does not have enough funds (should be handled by treasury management).

## Cross-Pallet Integration
- Requires a trait implementation from the Sample Metadata Pallet:
  - `approved_sample_count(who: &AccountId) -> u32`: Returns the number of approved samples for a user.
  - `all_users() -> Vec<AccountId>`: Returns all user accounts (for off-chain worker enumeration).

## Off-Chain Worker Automation
- The off-chain worker runs at the configured interval (`RewardPeriod`).
- It collects all eligible users and submits an unsigned transaction to trigger `distribute_rewards`.
- Ensures rewards are distributed automatically and only once per period.

## Usage Examples
### Manual Reward Distribution (for testing)
```sh
# As root, call distribute_rewards with a list of eligible users
```

### Automated Distribution
- The off-chain worker will handle reward distribution at the configured interval.

## Testing Guidelines
- Unit and integration tests should cover:
  - Eligibility calculation
  - Idempotency (no double rewards)
  - Event emission
  - Off-chain worker scheduling
  - Error handling (e.g., insufficient funds, already rewarded)

## Security & Best Practices
- Only root or off-chain worker should be able to call `distribute_rewards`.
- Ensure the reward pool is funded and managed securely.
- Monitor events for auditing and analytics.

---
For more details, see the Rust doc comments in `pallets/content-rewards/src/lib.rs`. 

## Related Documentation

*   [Main EchoChain Project README](../README.md)
*   [EchoChain Documentation and Development Plan](./EchoChain_Documentation_and_Development_Plan.md)
*   [Architecture Overview](./architecture.md) 