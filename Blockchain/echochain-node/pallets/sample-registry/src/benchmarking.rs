//! Benchmarking for sample-registry pallet
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    // TODO: Implement real benchmarks for all extrinsics. See issue #123.
    unimplemented_benchmark {
        unimplemented!("Real benchmarks not yet implemented. See issue #123.");
    }
} 