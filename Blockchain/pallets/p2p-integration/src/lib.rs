#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch::DispatchResult,
    traits::Get,
};
use frame_system::ensure_signed;
use sp_std::prelude::*;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    
    /// The PoC pallet instance
    type Poc: proof_of_contribution::Config;
    
    /// Reward rate for storage contributions (per MB per block)
    type StorageRewardRate: Get<u128>;
    
    /// Reward rate for bandwidth contributions (per MB transferred)
    type BandwidthRewardRate: Get<u128>;
}

decl_storage! {
    trait Store for Module<T: Config> as P2PIntegration {
        /// Storage contributions by account (MB-hours)
        pub StorageContributions get(fn storage_contributions):
            map hasher(blake2_128_concat) T::AccountId => u64;
            
        /// Bandwidth contributions by account (MB transferred)
        pub BandwidthContributions get(fn bandwidth_contributions):
            map hasher(blake2_128_concat) T::AccountId => u64;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        /// New storage contribution recorded [who, amount_mb, duration_blocks]
        StorageContributed(AccountId, u64, u32),
        /// New bandwidth contribution recorded [who, amount_mb]
        BandwidthContributed(AccountId, u64),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        /// Contribution amount overflow
        ContributionOverflow,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Record storage contribution (called by P2P node)
        #[weight = 10_000]
        pub fn record_storage(
            origin,
            amount_mb: u64,
            duration_blocks: u32
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            // Calculate and record contribution
            let contribution = amount_mb.checked_mul(duration_blocks.into())
                .ok_or(Error::<T>::ContributionOverflow)?;
                
            <StorageContributions<T>>::mutate(&who, |v| {
                *v = v.checked_add(contribution)
                    .ok_or(Error::<T>::ContributionOverflow)?;
                Ok(())
            })?;
            
            // Convert to PoC contribution units and record
            let poc_contribution = contribution.checked_mul(T::StorageRewardRate::get().into())
                .ok_or(Error::<T>::ContributionOverflow)?;
                
            proof_of_contribution::Module::<T::Poc>::add_network_contribution(
                &who,
                poc_contribution.into()
            )?;
            
            Self::deposit_event(RawEvent::StorageContributed(who, amount_mb, duration_blocks));
            Ok(())
        }

        /// Record bandwidth contribution (called by P2P node)
        #[weight = 10_000]
        pub fn record_bandwidth(
            origin,
            amount_mb: u64
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            <BandwidthContributions<T>>::mutate(&who, |v| {
                *v = v.checked_add(amount_mb)
                    .ok_or(Error::<T>::ContributionOverflow)?;
                Ok(())
            })?;
            
            // Convert to PoC contribution units and record
            let poc_contribution = amount_mb.checked_mul(T::BandwidthRewardRate::get().into())
                .ok_or(Error::<T>::ContributionOverflow)?;
                
            proof_of_contribution::Module::<T::Poc>::add_network_contribution(
                &who,
                poc_contribution.into()
            )?;
            
            Self::deposit_event(RawEvent::BandwidthContributed(who, amount_mb));
            Ok(())
        }
    }
}
