#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

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
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ComputeTaskCreated(u32),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn create_compute_task(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            // This is a stub. Logic for creating a compute task will be added here.
            Self::deposit_event(Event::ComputeTaskCreated(task_id));
            Ok(())
        }
    }

    pub trait ComputeInterface<AccountId> {
        fn create_task(who: AccountId, task_id: u32) -> DispatchResult;
    }

    impl<T: Config> ComputeInterface<T::AccountId> for Pallet<T> {
        fn create_task(who: T::AccountId, task_id: u32) -> DispatchResult {
            // This is a stub. The actual implementation would create a compute task.
            Pallet::<T>::deposit_event(Event::ComputeTaskCreated(task_id));
            Ok(())
        }
    }
}