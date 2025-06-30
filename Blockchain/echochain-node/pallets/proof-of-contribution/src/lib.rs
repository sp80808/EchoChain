#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{Currency, ReservableCurrency};
	use sp_runtime::traits::UniqueSaturatedInto;
	use sp_std::prelude::*;

	use pallet_sample_registry::{SampleStatus, Samples as SampleRegistrySamples};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config + pallet_timestamp::Config {
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

		/// The period in blocks for distributing network rewards.
		#[pallet::constant]
		type NetworkRewardPeriod: Get<Self::BlockNumber>;

		/// The account ID for the treasury where rewards are minted from.
		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;
	}

	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::storage]
	#[pallet::getter(fn last_content_reward_block)]
	pub(super) type LastContentRewardBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_network_reward_block)]
	pub(super) type LastNetworkRewardBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Content rewards have been distributed.
		ContentRewardsDistributed { block_number: T::BlockNumber, rewarded_creators: u32, total_amount: BalanceOf<T> },
		/// Network rewards have been distributed.
		NetworkRewardsDistributed { block_number: T::BlockNumber, total_amount: BalanceOf<T> },
		/// Network contribution reported.
		NetworkContributionReported { who: T::AccountId, storage_bytes: u64, bandwidth_bytes: u64 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No samples found for content reward.
		NoEligibleCreators,
		/// Cannot reward due to arithmetic overflow.
		RewardOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let mut weight = T::DbWeight::get().reads(1);

			// Content Rewards (monthly check - simplified to daily for testing)
			if (block_number % T::NetworkRewardPeriod::get() == 0) {
				weight = weight.saturating_add(Self::distribute_content_rewards());
			}

			// Network Rewards (periodic check)
			if (block_number % T::NetworkRewardPeriod::get() == 0) {
				weight = weight.saturating_add(Self::distribute_network_rewards());
			}
			weight
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
			Self::deposit_event(Event::NetworkContributionReported { who, storage_bytes, bandwidth_bytes });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Distribute content rewards to eligible creators.
		fn distribute_content_rewards() -> Weight {
			let mut rewarded_creators = 0;
			let mut total_reward_amount: BalanceOf<T> = Zero::zero();
			let current_block_number = <frame_system::Pallet<T>>::block_number();

			// Iterate through all samples and group by creator to count approved samples
			let mut creator_approved_samples: sp_std::collections::btree_map::BTreeMap<T::AccountId, u32> = 
				sp_std::collections::btree_map::BTreeMap::new();

			for (_sample_id, sample_metadata) in SampleRegistrySamples::<T>::iter() {
				if sample_metadata.status == SampleStatus::Approved {
					*creator_approved_samples.entry(sample_metadata.owner).or_insert(0) += 1;
				}
			}

			for (creator, count) in creator_approved_samples.into_iter() {
				if count >= T::MinSamplesForContentReward::get() {
					let reward = T::ContentRewardAmount::get();
					// Mint and transfer tokens from treasury to creator
					T::Currency::deposit_creating(&creator, reward);
					rewarded_creators += 1;
					total_reward_amount = total_reward_amount.saturating_add(reward);
				}
			}

			Self::deposit_event(Event::ContentRewardsDistributed {
				block_number: current_block_number,
				rewarded_creators,
				total_amount,
			});
			T::DbWeight::get().reads_writes(creator_approved_samples.len().unique_saturated_into(), rewarded_creators.unique_saturated_into())
		}

		/// Distribute network rewards based on reported contributions.
		fn distribute_network_rewards() -> Weight {
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let total_network_reward_pool: BalanceOf<T> = T::Currency::free_balance(&T::TreasuryAccount::get());
			
			// For simplicity, we'll just log a message for now.
			// In a real implementation, this would involve more complex logic
			// to calculate and distribute rewards based on reported storage/bandwidth.
			Self::deposit_event(Event::NetworkRewardsDistributed {
				block_number: current_block_number,
				total_amount: total_network_reward_pool,
			});
			T::DbWeight::get().reads_writes(1, 1)
		}
	}
}
