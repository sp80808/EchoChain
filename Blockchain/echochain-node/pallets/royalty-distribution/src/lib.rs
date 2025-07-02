#![cfg_attr(not(feature = "std"), no_std)]

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
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// Currency type for payments
        type Currency: Currency<Self::AccountId>;
    }

    /// Storage for royalty distribution percentages
    #[pallet::storage]
    #[pallet::getter(fn royalty_percentages)]
    pub type RoyaltyPercentages<T> = StorageValue<_, (u8, u8, u8), ValueQuery>;

    /// Storage for NFT metadata (off-chain reference)
    #[pallet::storage]
    #[pallet::getter(fn nft_metadata)]
    pub type NftMetadata<T> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<u8>, ValueQuery>;

    /// Events for the royalty distribution pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Royalties distributed [creator, contributor, liquidity_pool]
        RoyaltiesDistributed(T::AccountId, T::AccountId, T::AccountId),
        /// NFT registered [nft_id, owner]
        NftRegistered(T::Hash, T::AccountId),
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

            // Calculate amounts for each recipient
            let creator_amount = amount * creator_percent.into() / 100u32.into();
            let contributor_amount = amount * contributor_percent.into() / 100u32.into();
            let liquidity_amount = amount * liquidity_percent.into() / 100u32.into();

            // Transfer funds
            T::Currency::transfer(&who, &creator, creator_amount, ExistenceRequirement::KeepAlive)?;
            T::Currency::transfer(&who, &contributor, contributor_amount, ExistenceRequirement::AllowDeath)?;
            T::Currency::transfer(&who, &liquidity_pool, liquidity_amount, ExistenceRequirement::AllowDeath)?;

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