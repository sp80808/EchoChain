use crate::{Config, Error, PartialJobRegistrationForMarketplace};
use frame_support::{pallet_prelude::DispatchError, sp_runtime::FixedU128, weights::Weight};
use pallet_echochain::{JobRegistrationFor, PoolId};
use crate::poc;
use crate::p2p;

/// Trait used to lookup the manager of a given processor account.
pub trait ManagerProvider<T: frame_system::Config> {
	fn manager_of(owner: &T::AccountId) -> Result<T::AccountId, DispatchError>;
}

/// Trait used to lookup the time a processor was last seen, i.e. sent a heartbeat.
pub trait ProcessorInfoProvider<T: frame_system::Config + crate::Config> {
	fn last_seen(processor: &T::AccountId) -> Option<u128>;
	fn processor_version(processor: &T::AccountId) -> Option<T::ProcessorVersion>;
	fn last_processor_metric(processor: &T::AccountId, pool_id: PoolId) -> Option<FixedU128>;
}

/// Manages each job's budget by reserving/unreserving rewards that are externally strored, e.g. on a pallet account in `pallet_balances`.
/// Manages storage capacity reservations for job executions
pub trait StorageTracker<T: Config> {
	/// Verifies sufficient storage capacity is available
	///
	/// # Arguments
	/// * `source` - The account requesting storage
	/// * `registration` - Partial job registration details
	///
	/// # Returns
	/// - Ok(()) if sufficient capacity available
	/// - Err(Error) if capacity check fails
	///
	/// # Notes
	/// - Does not actually reserve capacity, just checks availability
	fn check(
		source: &T::AccountId,
		registration: &PartialJobRegistrationForMarketplace<T>,
	) -> Result<(), Error<T>>;

	/// Reserves storage capacity for a job
	///
	/// # Arguments
	/// * `source` - The account requesting storage
	/// * `registration` - Full job registration details
	///
	/// # Returns
	/// - Ok(()) on successful reservation
	/// - Err(Error) if reservation fails
	///
	/// # Side Effects
	/// - Reduces available capacity for the source account
	fn lock(source: &T::AccountId, registration: &JobRegistrationFor<T>) -> Result<(), Error<T>>;

	/// Releases storage capacity after job completion
	///
	/// # Arguments
	/// * `source` - The account that reserved storage
	/// * `registration` - Completed job details
	///
	/// # Returns
	/// - Ok(()) on successful release
	/// - Err(Error) if release fails
	///
	/// # Side Effects
	/// - Increases available capacity for the source account
	fn unlock(source: &T::AccountId, registration: &JobRegistrationFor<T>) -> Result<(), Error<T>>;
}

/// Weight functions needed for pallet_echochain_marketplace.
pub trait WeightInfo {
	fn advertise() -> Weight;
	fn delete_advertisement() -> Weight;
	fn report() -> Weight;
	fn propose_matching(x: u32) -> Weight;
	fn propose_execution_matching(x: u32) -> Weight;
	fn acknowledge_match() -> Weight;
	fn acknowledge_execution_match() -> Weight;
	fn finalize_job() -> Weight;
	fn finalize_jobs(x: u32) -> Weight;
	fn cleanup_storage(x: u32) -> Weight;
	fn cleanup_assignments(x: u32) -> Weight;
	fn edit_script() -> Weight;
	fn transfer_editor() -> Weight;
	fn deploy() -> Weight;
}

/// Handles reward payments and refunds for completed marketplace jobs
pub trait RewardManager<T: frame_system::Config> {
	/// Pays out reward for a completed job
	///
	/// # Arguments
	/// * `job_id` - The ID of the completed job
	/// * `amount` - The reward amount to pay out
	/// * `recipient` - The account to receive the reward
	///
	/// # Returns
	/// - Ok(()) on successful payment
	/// - Err(()) if payment fails (insufficient funds, invalid job, etc)
	///
	/// # Example
	/// ```rust
	/// reward_manager.pay_reward(&job_id, reward_amount, &creator)?;
	/// ```
	fn pay_reward(job_id: &u64, amount: T::Balance, recipient: &T::AccountId) -> Result<(), ()>;

	/// Processes refund for a cancelled or failed job
	///
	/// # Arguments
	/// * `job_id` - The ID of the job to refund
	///
	/// # Returns
	/// - Ok(Balance) refund amount on success
	/// - Err(()) if refund fails (invalid job, already refunded, etc)
	///
	/// # Notes
	/// - Refunds are paid back to the original job funder
	/// - Partial refunds may occur based on job progress
	fn refund(job_id: &u64) -> Result<T::Balance, ()>;
}

/// Provides reputation and status information about processors
pub trait ProcessorInfoProvider<T: frame_system::Config> {
	/// Retrieves processor metadata including reputation score and online status
	///
	/// # Arguments
	/// * `account` - The processor account to query
	///
	/// # Returns
	/// - Some(ProcessorInfo) if processor exists
	/// - None if processor is unknown
	///
	/// # Example
	/// ```rust
	/// if let Some(info) = processor_info(processor) {
	///     if info.is_online {
	///         // Consider for job assignment
	///     }
	/// }
	/// ```
	fn get_processor_info(account: &T::AccountId) -> Option<ProcessorInfo>;
}

pub struct ProcessorInfo {
	pub reputation: u32,
	pub is_online: bool,
	// Add more fields as needed for P2P integration
}

impl<T: frame_system::Config> RewardManager<T> for () {
	fn pay_reward(job_id: &u64, amount: T::Balance, recipient: &T::AccountId) -> Result<(), ()> {
		// Call PoC pallet to distribute reward
		poc::Pallet::<T>::distribute_reward(job_id, amount, recipient).map_err(|_| ())
	}
	fn refund(job_id: &u64) -> Result<T::Balance, ()> {
		// Call PoC pallet to refund reward
		poc::Pallet::<T>::refund_reward(job_id).map_err(|_| ())
	}
}

impl<T: frame_system::Config> ProcessorInfoProvider<T> for () {
	fn get_processor_info(account: &T::AccountId) -> Option<ProcessorInfo> {
		// Call P2P pallet to fetch processor info
		p2p::Pallet::<T>::get_processor_info(account)
	}
}
