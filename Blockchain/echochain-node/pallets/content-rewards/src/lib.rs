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
    use pallet_sample_registry::SampleInterface;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MinApprovedSamples: Get<u32>;
        type RewardAmount: Get<BalanceOf<Self>>;
        type SampleRegistry: SampleInterface<Self::AccountId>;
        /// Currency type for content reward payments
        #[pallet::constant]
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RewardDistributed(T::AccountId, BalanceOf<T>),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn distribute_rewards(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            let reward_amount = T::RewardAmount::get();
            let min_samples = T::MinApprovedSamples::get();

            // Collect eligible accounts to avoid borrowing issues
            let eligible_accounts: Vec<T::AccountId> = frame_system::Account::<T>::iter()
                .filter_map(|(account, _)| {
                    if T::SampleRegistry::get_approved_sample_count(&account) >= min_samples {
                        Some(account)
                    } else {
                        None
                    }
                })
                .collect();

            for account in eligible_accounts {
                // Actual balance transfer implementation
                T::Currency::deposit_creating(&account, reward_amount);
                Self::deposit_event(Event::RewardDistributed(account, reward_amount));
            }

            Ok(())
        }
    }
}
