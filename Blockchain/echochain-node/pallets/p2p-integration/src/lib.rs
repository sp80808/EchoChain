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
    use pallet_network_rewards::NetworkRewardsInterface;
    use pallet_echochain_compute::ComputeInterface;
    use pallet_echochain_marketplace::MarketplaceInterface;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type NetworkRewards: NetworkRewardsInterface<Self::AccountId>;
        type EchochainCompute: ComputeInterface<Self::AccountId>;
        type EchochainMarketplace: MarketplaceInterface<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        P2PNodeRegistered(T::AccountId),
        P2PActivityReported(T::AccountId, u64, u64),
        ComputeJobTriggered(T::AccountId, u64, u32),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_p2p_node(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // This is a stub. Logic to register a P2P node will be added here.
            Self::deposit_event(Event::P2PNodeRegistered(who));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn report_p2p_activity(origin: OriginFor<T>, bytes_uploaded: u64, bytes_downloaded: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;
            T::NetworkRewards::submit_report(who.clone(), bytes_uploaded, bytes_downloaded)?;
            Self::deposit_event(Event::P2PActivityReported(who, bytes_uploaded, bytes_downloaded));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn trigger_compute_job(origin: OriginFor<T>, job_id: u64, task_id: u32, job_details: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            T::EchochainMarketplace::post_job(who.clone(), job_id, job_details)?;
            T::EchochainCompute::create_task(who.clone(), task_id)?;
            Self::deposit_event(Event::ComputeJobTriggered(who, job_id, task_id));
            Ok(())
        }
    }
}
