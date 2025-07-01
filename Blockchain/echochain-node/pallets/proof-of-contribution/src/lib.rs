#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency, ExistenceRequirement, Get},
		BoundedVec,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{traits::{UniqueSaturatedInto, Saturating}, Perbill};
	use sp_std::{prelude::*, vec::Vec, collections::btree_map::BTreeMap};

	use pallet_sample_registry::{SampleStatus, Samples as SampleRegistrySamples};
	use frame_system::offchain::{SubmitTransaction, SendTransactionTypes};

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct SeedingReport<BlockNumber> {
		pub bytes_uploaded: u64,
		pub bytes_downloaded: u64,
		pub timestamp: BlockNumber,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config + pallet_timestamp::Config + SendTransactionTypes<Call<Self>> {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency mechanism.
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// The amount of ECHO tokens to reward for content contribution.
		#[pallet::constant]
		type ContentRewardAmount: Get<BalanceOf<Self>>;

		/// The minimum number of approved samples required for content reward.
		#[pallet::constant]
		type MinSamplesForContentReward: Get<u32>;

		/// The period in blocks for distributing rewards (both content and network).
		#[pallet::constant]
		type RewardPeriod: Get<Self::BlockNumber>;

		/// The account ID for the treasury where rewards are minted from.
		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;

		/// Total reward for network contribution per period.
		#[pallet::constant]
		type TotalNetworkRewardPerPeriod: Get<BalanceOf<Self>>;

		/// Minimum bytes uploaded for network contribution eligibility.
		#[pallet::constant]
		type MinBytesUploadedForNetworkReward: Get<u64>;
	}

	#[pallet::storage]
	#[pallet::getter(fn last_reward_block)]
	pub(super) type LastRewardBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn seeding_reports)]
	pub type SeedingReports<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		SeedingReport<T::BlockNumber>,
		OptionQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Content rewards have been distributed.
		ContentRewardsDistributed { block_number: T::BlockNumber, rewarded_creators: u32, total_amount: BalanceOf<T> },
		/// Network rewards have been distributed.
		NetworkRewardsDistributed { block_number: T::BlockNumber, total_amount: BalanceOf<T> },
		/// Network contribution reported.
		NetworkContributionReported { who: T::AccountId, storage_bytes: u64, bandwidth_bytes: u64 },
		/// Reward distributed to a user.
		RewardDistributed { user: T::AccountId, amount: BalanceOf<T> },
		/// Network reward distributed to a user.
		NetworkRewardDistributedToUser { user: T::AccountId, amount: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No samples found for content reward.
		NoEligibleCreators,
		/// Cannot reward due to arithmetic overflow.
		RewardOverflow,
		/// User has already reported for this period.
		AlreadyReportedThisPeriod,
		/// Not enough contribution to be eligible for network rewards.
		NotEnoughContribution,
		/// Reward pool is empty.
		RewardPoolEmpty,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let mut weight = T::DbWeight::get().reads(1);

			if (block_number % T::RewardPeriod::get()).is_zero() {
				if let Some(last) = Self::last_reward_block() {
					if block_number <= last { return weight; }
				}
				
				// Trigger content rewards distribution via offchain worker
				let content_reward_call = Call::<T>::distribute_content_rewards_unsigned {};
				let _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(content_reward_call.into());
				
				// Trigger network rewards distribution via offchain worker
				let network_reward_call = Call::<T>::distribute_network_rewards_unsigned {};
				let _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(network_reward_call.into());

				LastRewardBlock::<T>::put(block_number);
			}
			weight
		}

		fn offchain_worker(now: T::BlockNumber) {
			// This offchain worker is triggered by on_initialize to distribute rewards
			// The actual distribution logic is in the callable functions.
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Report network contribution (storage and bandwidth).
		#[pallet::weight(10_000 + T::DbWeight::writes(1).ref_time())]
		pub fn report_network_contribution(
			origin: OriginFor<T>,
			storage_bytes: u64,
			bandwidth_bytes: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(bandwidth_bytes >= T::MinBytesUploadedForNetworkReward::get(), Error::<T>::NotEnoughContribution);

			let now = <frame_system::Pallet<T>>::block_number();
			// Only one report per user per period
			if let Some(report) = <SeedingReports<T>>::get(&who) {
				if now.saturating_sub(report.timestamp) < T::RewardPeriod::get() {
					return Err(Error::<T>::AlreadyReportedThisPeriod.into());
				}
			}

			<SeedingReports<T>>::insert(
				&who,
				SeedingReport {
					bytes_uploaded: storage_bytes, // Renamed for consistency with network-rewards pallet
					bytes_downloaded: bandwidth_bytes, // Renamed for consistency with network-rewards pallet
					timestamp: now,
				},
			);
			Self::deposit_event(Event::NetworkContributionReported { who, storage_bytes, bandwidth_bytes });
			Ok(())
		}

		/// Distribute content rewards to eligible creators (called by offchain worker).
		#[pallet::weight(10_000 + T::DbWeight::reads_writes(100, 100).ref_time())] // Placeholder weight
		pub fn distribute_content_rewards_unsigned(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?; // Must be called by offchain worker

			let mut rewarded_creators = 0;
			let mut total_reward_amount: BalanceOf<T> = Zero::zero();
			let current_block_number = <frame_system::Pallet<T>>::block_number();

			let mut creator_approved_samples: BTreeMap<T::AccountId, u32> =
				BTreeMap::new();

			for (_sample_id, sample_metadata) in SampleRegistrySamples::<T>::iter() {
				if sample_metadata.status == SampleStatus::Approved {
					*creator_approved_samples.entry(sample_metadata.owner).or_insert(0) += 1;
				}
			}

			for (creator, count) in creator_approved_samples.into_iter() {
				if count >= T::MinSamplesForContentReward::get() {
					let reward = T::ContentRewardAmount::get();
					T::Currency::deposit_creating(&creator, reward);
					rewarded_creators += 1;
					total_reward_amount = total_reward_amount.saturating_add(reward);
					Self::deposit_event(Event::RewardDistributed { user: creator, amount: reward });
				}
			}

			Self::deposit_event(Event::ContentRewardsDistributed {
				block_number: current_block_number,
				rewarded_creators,
				total_amount,
			});
			Ok(().into())
		}

		/// Distribute network rewards based on reported contributions (called by offchain worker).
		#[pallet::weight(10_000 + T::DbWeight::reads_writes(100, 100).ref_time())] // Placeholder weight
		pub fn distribute_network_rewards_unsigned(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?; // Must be called by offchain worker

			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let total_network_reward_pool: BalanceOf<T> = T::TotalNetworkRewardPerPeriod::get();
			
			let mut total_uploaded: u64 = 0;
			let mut eligible_users: Vec<(T::AccountId, u64)> = Vec::new();

			for (user, report) in <SeedingReports<T>>::iter() {
				if report.bytes_uploaded >= T::MinBytesUploadedForNetworkReward::get() {
					total_uploaded += report.bytes_uploaded;
					eligible_users.push((user, report.bytes_uploaded));
				}
			}

			if total_uploaded == 0 {
				return Ok(().into()); // No eligible users
			}

			for (user, uploaded) in eligible_users {
				let share = Perbill::from_rational(uploaded, total_uploaded);
				let reward = share * total_network_reward_pool;
				T::Currency::transfer(
					&T::TreasuryAccount::get(),
					&user,
					reward,
					ExistenceRequirement::KeepAlive,
				)?;
				Self::deposit_event(Event::NetworkRewardDistributedToUser { user, amount: reward });
			}

			<SeedingReports<T>>::remove_all(None); // Clear reports for next period

			Self::deposit_event(Event::NetworkRewardsDistributed {
				block_number: current_block_number,
				total_amount: total_network_reward_pool,
			});
			Ok(().into())
		}
	}
}
