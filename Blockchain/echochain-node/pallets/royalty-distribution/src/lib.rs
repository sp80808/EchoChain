#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

//! # Royalty Distribution Pallet
//!
//! A Substrate pallet for managing dynamic royalty distributions with configurable splits.
//!
//! ## Overview
//!
//! This pallet provides functionality for:
//! - Configurable royalty percentage splits (default 70/10/20)
//! - NFT registration with minimal on-chain storage
//! - Automated royalty distribution to multiple parties
//!
//! ## Terminology
//!
//! - **Creator Royalty**: The primary recipient (70% by default)
//! - **Contributor Reward**: Secondary recipient (10% by default)
//! - **Liquidity Pool**: Protocol share (20% by default)
//! - **NFT Metadata**: Off-chain reference data stored on IPFS
//!
//! ## Implementation
//!
//! Royalties are distributed atomically in a single transaction to minimize gas costs.
//! NFT metadata is stored minimally on-chain with full content stored off-chain.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// Currency type for royalty payments
        #[pallet::constant]
        type Currency: Currency<Self::AccountId>;
        
        /// The minimum interval between distribution cycles (in milliseconds, approximating a week)
        #[pallet::constant]
        type DistributionInterval: Get<Self::Moment>;
    }

    /// Storage for royalty distribution percentages
    #[pallet::storage]
    #[pallet::getter(fn royalty_percentages)]
    pub type RoyaltyPercentages<T> = StorageValue<_, (u8, u8, u8), ValueQuery>;

    /// Storage for NFT metadata (off-chain reference)
    #[pallet::storage]
    #[pallet::getter(fn nft_metadata)]
    pub type NftMetadata<T> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<u8>, ValueQuery>;

    /// Storage for usage events of NFTs
    #[pallet::storage]
    #[pallet::getter(fn usage_events)]
    pub type UsageEvents<T> = StorageDoubleMap<_, Blake2_128Concat, T::Hash, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    /// Storage for the last distribution timestamp
    #[pallet::storage]
    #[pallet::getter(fn last_distribution)]
    pub type LastDistribution<T> = StorageValue<_, T::Moment, ValueQuery>;

    /// Events for the royalty distribution pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Royalties distributed [creator, contributor, liquidity_pool]
        RoyaltiesDistributed(T::AccountId, T::AccountId, T::AccountId),
        /// NFT registered [nft_id, owner]
        NftRegistered(T::Hash, T::AccountId),
        /// Usage event recorded [nft_id, user]
        UsageRecorded(T::Hash, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Invalid royalty percentages (must sum to 100)
        InvalidRoyaltySplit,
        /// NFT already registered
        NftAlreadyExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Initialize royalty distribution percentages
        #[pallet::weight(10_000)]
        pub fn initialize_royalties(
            origin: OriginFor<T>,
            creator_percent: u8,
            contributor_percent: u8,
            liquidity_percent: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            ensure!(
                creator_percent + contributor_percent + liquidity_percent == 100,
                Error::<T>::InvalidRoyaltySplit
            );

            RoyaltyPercentages::<T>::put((creator_percent, contributor_percent, liquidity_percent));
            Ok(())
        }

        /// Register a new NFT with off-chain metadata
        #[pallet::weight(10_000)]
        pub fn register_nft(
            origin: OriginFor<T>,
            nft_id: T::Hash,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                !NftMetadata::<T>::contains_key(nft_id),
                Error::<T>::NftAlreadyExists
            );

            NftMetadata::<T>::insert(nft_id, metadata);
            Self::deposit_event(Event::NftRegistered(nft_id, who));
            Ok(())
        }

        /// Record a usage event for an NFT
        #[pallet::weight(5_000)]
        pub fn record_usage(
            origin: OriginFor<T>,
            nft_id: T::Hash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                NftMetadata::<T>::contains_key(nft_id),
                Error::<T>::NftAlreadyExists
            );

            let current_count = UsageEvents::<T>::get(nft_id, &who);
            UsageEvents::<T>::insert(nft_id, &who, current_count.saturating_add(1));
            Self::deposit_event(Event::UsageRecorded(nft_id, who));
            Ok(())
        }

        /// Trigger automated royalty distribution based on usage events if interval has passed
        #[pallet::weight(30_000)]
        pub fn trigger_distribution(
            origin: OriginFor<T>,
            amount_per_usage: BalanceOf<T>,
            creator: T::AccountId,
            contributor: T::AccountId,
            liquidity_pool: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let now = pallet_timestamp::Pallet::<T>::get();
            let last_dist = LastDistribution::<T>::get();
            let interval = T::DistributionInterval::get();

            ensure!(
                now >= last_dist + interval,
                Error::<T>::InvalidRoyaltySplit
            );

            let (creator_percent, contributor_percent, liquidity_percent) = RoyaltyPercentages::<T>::get();
            let total_percent = creator_percent as u32 + contributor_percent as u32 + liquidity_percent as u32;
            ensure!(total_percent == 100, Error::<T>::InvalidRoyaltySplit);

            let mut total_amount = BalanceOf::<T>::zero();
            for (nft_id, user_counts) in UsageEvents::<T>::iter() {
                for (_, count) in user_counts {
                    let usage_amount = amount_per_usage.checked_mul(&count.into()).ok_or(ArithmeticError::Overflow)?;
                    total_amount = total_amount.checked_add(&usage_amount).ok_or(ArithmeticError::Overflow)?;
                }
            }

            if total_amount > BalanceOf::<T>::zero() {
                let one_percent = total_amount.checked_div(&100u32.into()).ok_or(ArithmeticError::Overflow)?;
                let creator_amount = one_percent.checked_mul(&creator_percent.into()).ok_or(ArithmeticError::Overflow)?;
                let contributor_amount = one_percent.checked_mul(&contributor_percent.into()).ok_or(ArithmeticError::Overflow)?;
                let liquidity_amount = total_amount
                    .checked_sub(&creator_amount)
                    .and_then(|v| v.checked_sub(&contributor_amount))
                    .ok_or(ArithmeticError::Overflow)?;

                frame_support::storage::with_transaction(|| {
                    T::Currency::transfer(&who, &creator, creator_amount, ExistenceRequirement::KeepAlive)?;
                    T::Currency::transfer(&who, &contributor, contributor_amount, ExistenceRequirement::AllowDeath)?;
                    T::Currency::transfer(&who, &liquidity_pool, liquidity_amount, ExistenceRequirement::AllowDeath)?;
                    Ok(())
                })?;

                LastDistribution::<T>::put(now);
                Self::deposit_event(Event::RoyaltiesDistributed(creator, contributor, liquidity_pool));
            }

            Ok(())
        }

        /// Distribute royalties according to configured percentages
        #[pallet::weight(20_000)]
        pub fn distribute_royalties(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            creator: T::AccountId,
            contributor: T::AccountId,
            liquidity_pool: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (creator_percent, contributor_percent, liquidity_percent) = RoyaltyPercentages::<T>::get();

            // Gas optimization: Use checked arithmetic to prevent overflows
            let total_percent = creator_percent as u32 + contributor_percent as u32 + liquidity_percent as u32;
            ensure!(total_percent == 100, Error::<T>::InvalidRoyaltySplit);

            // Calculate amounts using single multiplication and division
            let one_percent = amount.checked_div(&100u32.into()).ok_or(ArithmeticError::Overflow)?;
            let creator_amount = one_percent.checked_mul(&creator_percent.into()).ok_or(ArithmeticError::Overflow)?;
            let contributor_amount = one_percent.checked_mul(&contributor_percent.into()).ok_or(ArithmeticError::Overflow)?;
            let liquidity_amount = amount
                .checked_sub(&creator_amount)
                .and_then(|v| v.checked_sub(&contributor_amount))
                .ok_or(ArithmeticError::Overflow)?;

            // Batch transfers to minimize storage operations
            frame_support::storage::with_transaction(|| {
                T::Currency::transfer(&who, &creator, creator_amount, ExistenceRequirement::KeepAlive)?;
                T::Currency::transfer(&who, &contributor, contributor_amount, ExistenceRequirement::AllowDeath)?;
                T::Currency::transfer(&who, &liquidity_pool, liquidity_amount, ExistenceRequirement::AllowDeath)?;
                Ok(())
            })?;

            Self::deposit_event(Event::RoyaltiesDistributed(creator, contributor, liquidity_pool));
            Ok(())
        }
    }

    // Helper implementation
    impl<T: Config> Pallet<T> {
        /// Get the current royalty distribution percentages
        pub fn get_royalty_percentages() -> (u8, u8, u8) {
            RoyaltyPercentages::<T>::get()
        }
    }
}
