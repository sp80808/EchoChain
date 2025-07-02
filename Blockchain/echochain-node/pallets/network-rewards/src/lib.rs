#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type TotalRewardPerPeriod: Get<BalanceOf<Self>>;
        /// Currency type for network reward payments
        #[pallet::constant]
        type Currency: Currency<Self::AccountId>;
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct SeedingReport {
        pub bytes_uploaded: u64,
        pub bytes_downloaded: u64,
    }

    #[pallet::storage]
    #[pallet::getter(fn reports)]
    pub type Reports<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, SeedingReport>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ReportSubmitted(T::AccountId),
        NetworkRewardDistributed(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyReportedThisPeriod,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn submit_report(origin: OriginFor<T>, bytes_uploaded: u64, bytes_downloaded: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if Reports::<T>::contains_key(&who) {
                return Err(Error::<T>::AlreadyReportedThisPeriod.into());
            }

            let report = SeedingReport {
                bytes_uploaded,
                bytes_downloaded,
            };

            Reports::<T>::insert(who.clone(), report);
            Self::deposit_event(Event::ReportSubmitted(who));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn distribute_network_rewards(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            let total_rewards = T::TotalRewardPerPeriod::get();
            let total_reports = Reports::<T>::iter().count();

            if total_reports == 0 {
                return Ok(());
            }

            let reward_per_report = total_rewards / (total_reports as u32).into();

            // Use a vector to collect account addresses to avoid borrowing issues
            let accounts: Vec<T::AccountId> = Reports::<T>::iter().map(|(account, _)| account).collect();

            for account in accounts {
                // Actual balance transfer implementation
                T::Currency::deposit_creating(&account, reward_per_report);
                Self::deposit_event(Event::NetworkRewardDistributed(account, reward_per_report));
            }

            // Clear reports for the next period
            let _ = Reports::<T>::clear(u32::MAX, None);

            Ok(())
        }
    }
}

pub trait NetworkRewardsInterface<AccountId> {
    fn submit_report(who: AccountId, bytes_uploaded: u64, bytes_downloaded: u64) -> DispatchResult;
}

impl<T: Config> NetworkRewardsInterface<T::AccountId> for Pallet<T> {
    fn submit_report(who: T::AccountId, bytes_uploaded: u64, bytes_downloaded: u64) -> DispatchResult {
        // This function is called by other pallets, so we don't need to check origin.
        if <Reports<T>>::contains_key(&who) {
            return Err(Error::<T>::AlreadyReportedThisPeriod.into());
        }

        let report = SeedingReport {
            bytes_uploaded,
            bytes_downloaded,
        };

        <Reports<T>>::insert(who.clone(), report);
        Pallet::<T>::deposit_event(Event::ReportSubmitted(who));
        Ok(())
    }
}
