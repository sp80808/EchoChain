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
    use pallet_sample_registry::SampleInterface;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MinApprovedSamples: Get<u32>;
        type RewardAmount: Get<u128>;
        type SampleRegistry: SampleInterface<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RewardDistributed(T::AccountId, u128),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn distribute_rewards(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;

            for (account, _) in frame_system::Account::<T>::iter() {
                if T::SampleRegistry::get_approved_sample_count(&account) >= T::MinApprovedSamples::get() {
                    // This is a stub for the actual reward distribution.
                    // The actual implementation would require a balance transfer.
                    Self::deposit_event(Event::RewardDistributed(account, T::RewardAmount::get()));
                }
            }

            Ok(())
        }
    }
}
