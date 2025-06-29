#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, Get},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::Perbill;
    use sp_std::vec::Vec;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct SeedingReport<BlockNumber> {
        pub bytes_uploaded: u64,
        pub bytes_downloaded: u64,
        pub timestamp: BlockNumber,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        #[pallet::constant]
        type RewardPoolAccount: Get<Self::AccountId>;
        #[pallet::constant]
        type TotalRewardPerPeriod: Get<BalanceOf<Self>>;
        #[pallet::constant]
        type MinBytesUploaded: Get<u64>;
        #[pallet::constant]
        type RewardPeriod: Get<Self::BlockNumber>;
    }

    #[pallet::storage]
    #[pallet::getter(fn reports)]
    pub type Reports<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        T::AccountId, 
        SeedingReport<T::BlockNumber>, 
        OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn last_reward_block)]
    pub type LastRewardBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ReportSubmitted { user: T::AccountId, uploaded: u64, downloaded: u64 },
        NetworkRewardDistributed { user: T::AccountId, amount: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyReportedThisPeriod,
        NotEnoughContribution,
        RewardPoolEmpty,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Users submit their seeding report for the current period.
        #[pallet::weight(10_000)]
        pub fn submit_report(
            origin: OriginFor<T>,
            bytes_uploaded: u64,
            bytes_downloaded: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(bytes_uploaded >= T::MinBytesUploaded::get(), Error::<T>::NotEnoughContribution);

            // Only one report per user per period
            ensure!(!Reports::<T>::contains_key(&who), Error::<T>::AlreadyReportedThisPeriod);

            let now = <frame_system::Pallet<T>>::block_number();
            Reports::<T>::insert(
                &who,
                SeedingReport {
                    bytes_uploaded,
                    bytes_downloaded,
                    timestamp: now,
                },
            );
            Self::deposit_event(Event::ReportSubmitted { user: who, uploaded: bytes_uploaded, downloaded: bytes_downloaded });
            Ok(())
        }

        /// Distribute network rewards to eligible users (root or off-chain worker only)
        #[pallet::weight(10_000)]
        pub fn distribute_network_rewards(origin: OriginFor<T>) -> DispatchResult {
            ensure_none(origin.clone()).or_else(|_| ensure_root(origin))?;

            let now = <frame_system::Pallet<T>>::block_number();
            if let Some(last) = Self::last_reward_block() {
                ensure!(now > last, Error::<T>::AlreadyReportedThisPeriod);
            }

            // Aggregate total uploaded bytes
            let mut total_uploaded: u64 = 0;
            let mut eligible_users: Vec<(T::AccountId, u64)> = Vec::new();

            for (user, report) in Reports::<T>::iter() {
                if report.bytes_uploaded >= T::MinBytesUploaded::get() {
                    total_uploaded += report.bytes_uploaded;
                    eligible_users.push((user, report.bytes_uploaded));
                }
            }

            if total_uploaded == 0 {
                return Ok(()); // No eligible users
            }

            let total_reward = T::TotalRewardPerPeriod::get();

            for (user, uploaded) in eligible_users {
                let share = Perbill::from_rational(uploaded, total_uploaded);
                let reward = share * total_reward;
                T::Currency::transfer(
                    &T::RewardPoolAccount::get(),
                    &user,
                    reward,
                    ExistenceRequirement::KeepAlive,
                )?;
                Self::deposit_event(Event::NetworkRewardDistributed { user, amount: reward });
            }

            // Clear reports for next period
            Reports::<T>::remove_all(None);
            LastRewardBlock::<T>::put(now);
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(now: T::BlockNumber) {
            if (now % T::RewardPeriod::get()).is_zero() {
                if let Some(last) = Self::last_reward_block() {
                    if now <= last { return; }
                }
                let call = Call::<T>::distribute_network_rewards {};
                let _ = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
            }
        }
    }
} 