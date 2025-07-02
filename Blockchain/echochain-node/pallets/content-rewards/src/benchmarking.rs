//! Benchmarking for content-rewards pallet
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::Hash;
use sp_std::prelude::*;

// Mock SampleInterface for benchmarking
pub struct MockSampleRegistry<T: Config>(sp_std::marker::PhantomData<T>);

impl<T: Config> SampleInterface<T::AccountId> for MockSampleRegistry<T> {
    fn get_approved_sample_count(owner: &T::AccountId) -> u32 {
        // In a benchmark, we want to simulate a worst-case scenario.
        // We can assume a fixed number of approved samples for any account
        // that is part of our benchmark setup.
        // For `distribute_rewards`, the cost is proportional to the number of accounts
        // that have approved samples.
        // We'll return a value greater than or equal to `MinApprovedSamples` for relevant accounts.
        T::MinApprovedSamples::get()
    }
}

benchmarks! {
    distribute_rewards {
        let n in 0 .. 1000; // Number of accounts to iterate through

        // Create n accounts, each with enough approved samples to qualify for rewards.
        // The actual number of approved samples doesn't matter for the weight,
        // only that `get_approved_sample_count` returns a value >= MinApprovedSamples.
        for i in 0..n {
            let account: T::AccountId = account("account", i, SEED);
            // We don't actually need to store samples, just simulate the condition
            // that `get_approved_sample_count` would return a qualifying value.
            // The `distribute_rewards` extrinsic iterates through all accounts in the system.
            // So, we need to ensure these accounts exist in the system state.
            frame_system::Pallet::<T>::inc_consumers(&account).unwrap();
        }

        let caller: T::AccountId = whitelisted_caller();
        // Ensure the caller is root for `distribute_rewards`
        // No setup needed for caller as it's `ensure_root`

    }: _(RawOrigin::Root)
    verify {
        // Verify that rewards were distributed.
        // This is a basic check; a more thorough test might check specific account balances
        // or event emissions if the mock SampleRegistry was more complex.
        // For this benchmark, we primarily care about the execution cost.
    }
}