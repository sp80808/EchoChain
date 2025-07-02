#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type TotalRewardPerPeriod: Get<u128>;
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
        NetworkRewardDistributed(T::AccountId, u128),
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
            let total_reports = Reports::<T>::iter().count() as u128;

            if total_reports == 0 {
                return Ok(());
            }

            let reward_per_report = total_rewards / total_reports;

            for (account, _) in Reports::<T>::iter() {
                // This is a stub for the actual reward distribution.
                // The actual implementation would require a balance transfer.
                Self::deposit_event(Event::NetworkRewardDistributed(account.clone(), reward_per_report));
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
