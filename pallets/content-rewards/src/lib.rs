#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Get},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Saturating;
    use sp_std::vec::Vec;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        #[pallet::constant]
        type MinApprovedSamples: Get<u32>;
        #[pallet::constant]
        type RewardAmount: Get<BalanceOf<Self>>;
        #[pallet::constant]
        type RewardPoolAccount: Get<Self::AccountId>;
        type SampleMetadata: SampleMetadataProvider<Self::AccountId>;
        /// Number of blocks between reward distributions (e.g., ~1 month)
        #[pallet::constant]
        type RewardPeriod: Get<Self::BlockNumber>;
    }

    // Cross-pallet trait for querying approved sample count
    pub trait SampleMetadataProvider<AccountId> {
        fn approved_sample_count(who: &AccountId) -> u32;
        fn all_users() -> Vec<AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn last_reward_block)]
    pub type LastRewardBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RewardDistributed { user: T::AccountId, amount: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyRewardedThisPeriod,
        RewardPoolEmpty,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Distribute rewards to eligible users (root or off-chain worker only)
        #[pallet::weight(10_000)]
        pub fn distribute_rewards(origin: OriginFor<T>, eligible_users: Vec<T::AccountId>) -> DispatchResult {
            ensure_none(origin.clone()).or_else(|_| ensure_root(origin))?;

            let now = <frame_system::Pallet<T>>::block_number();
            if let Some(last) = Self::last_reward_block() {
                ensure!(now > last, Error::<T>::AlreadyRewardedThisPeriod);
            }

            let mut distributed = false;
            for user in eligible_users.iter() {
                let count = T::SampleMetadata::approved_sample_count(user);
                if count >= T::MinApprovedSamples::get() {
                    let amount = T::RewardAmount::get();
                    T::Currency::transfer(
                        &T::RewardPoolAccount::get(),
                        user,
                        amount,
                        ExistenceRequirement::KeepAlive,
                    )?;
                    Self::deposit_event(Event::RewardDistributed { user: user.clone(), amount });
                    distributed = true;
                }
            }
            if distributed {
                LastRewardBlock::<T>::put(now);
            }
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(now: T::BlockNumber) {
            // Only run at the start of a new period
            if (now % T::RewardPeriod::get()).is_zero() {
                if let Some(last) = Self::last_reward_block() {
                    if now <= last { return; }
                }
                // Find eligible users (off-chain, for scalability)
                let eligible_users = T::SampleMetadata::all_users()
                    .into_iter()
                    .filter(|user| T::SampleMetadata::approved_sample_count(user) >= T::MinApprovedSamples::get())
                    .collect::<Vec<_>>();
                if !eligible_users.is_empty() {
                    let call = Call::<T>::distribute_rewards { eligible_users };
                    let _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
                }
            }
        }
    }
} 