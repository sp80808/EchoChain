//! Benchmarking for p2p-integration pallet
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    dummy_benchmark {
        let caller: T::AccountId = whitelisted_caller();
    }: {
        // No-op
    }
    verify {
        // No-op
    }
} 