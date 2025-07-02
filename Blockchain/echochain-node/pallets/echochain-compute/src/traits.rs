use frame_support::{
	pallet_prelude::{DispatchResult, Weight},
	sp_runtime::DispatchError,
};
use sp_runtime::{traits::Zero, Perquintill};

use crate::{Config, EpochOf};
use crate::poc;
use crate::p2p;

/// Handles compute reward distribution logic for processor nodes
///
/// This trait defines the interface for calculating and distributing rewards
/// based on processor metrics and participation in the network.
pub trait ComputeRewardDistributor<T: frame_system::Config> {
	/// Calculates the reward amount for a given epoch without transferring funds
	///
	/// # Arguments
	///
	/// * `ratio` - The ratio (0-100) of total reward pool to distribute this epoch
	/// * `epoch` - The global epoch number for reward calculation
	///
	/// # Returns
	/// - Ok(Balance) on successful calculation
	/// - Err(()) if calculation fails (overflow, invalid parameters)
	///
	/// # Example
	/// ```rust
	/// let reward = compute_reward(80, current_epoch)?;
	/// ```
	fn calculate_reward(ratio: u32, epoch: u64) -> Result<T::Balance, ()>;

	/// Distributes calculated reward to processor's manager account
	///
	/// # Arguments
	///
	/// * `processor` - The processor account that earned the reward
	/// * `amount` - The reward amount to distribute
	///
	/// # Returns
	/// - Ok(()) on successful distribution
	/// - Err(()) if distribution fails
	///
	/// # Notes
	/// - Rewards are paid to the manager account, not directly to processor
	/// - Uses pallet_balances for actual fund transfer
	fn distribute_reward(processor: &T::AccountId, amount: T::Balance) -> Result<(), ()>;

	/// Checks if a processor is eligible to receive rewards
	///
	/// # Arguments
	/// * `processor` - The processor account to check
	///
	/// # Returns
	/// - true if eligible (active, meets minimum requirements)
	/// - false if ineligible (slashed, offline, etc)
	///
	/// # Example
	/// ```rust
	/// if is_eligible(&processor) {
	///     // distribute reward
	/// }
	/// ```
	fn is_eligible_for_reward(processor: &T::AccountId) -> bool;
}

pub trait ManagerIdProvider<AccountId, ManagerId> {
	fn manager_of(account: &AccountId) -> Option<ManagerId>;
}

impl<T: Config<I>, I: 'static> ComputeRewardDistributor<T> for () {
	fn calculate_reward(ratio: u32, epoch: u64) -> Result<T::Balance, ()> {
		poc::Pallet::<T>::calculate_compute_reward(ratio, epoch).map_err(|_| ())
	}

	fn distribute_reward(processor: &T::AccountId, amount: T::Balance) -> Result<(), ()> {
		poc::Pallet::<T>::distribute_compute_reward(processor, amount).map_err(|_| ())
	}

	fn is_eligible_for_reward(processor: &T::AccountId) -> bool {
		poc::Pallet::<T>::is_eligible_for_compute_reward(processor)
	}
}

impl<AccountId, ManagerId> ManagerIdProvider<AccountId, ManagerId> for () {
	fn manager_of(account: &AccountId) -> Option<ManagerId> {
		p2p::Pallet::manager_of(account)
	}
}

pub trait WeightInfo {
	fn create_pool(x: u32) -> Weight;
	fn modify_pool_same_config() -> Weight;
	fn modify_pool_replace_config(x: u32) -> Weight;
	fn modify_pool_update_config(x: u32) -> Weight;
}

impl WeightInfo for () {
	fn create_pool(_x: u32) -> Weight {
		Weight::from_parts(10_000, 0)
	}

	fn modify_pool_same_config() -> Weight {
		Weight::from_parts(10_000, 0)
	}

	fn modify_pool_replace_config(_x: u32) -> Weight {
		Weight::from_parts(10_000, 0)
	}

	fn modify_pool_update_config(_x: u32) -> Weight {
		Weight::from_parts(10_000, 0)
	}
}
