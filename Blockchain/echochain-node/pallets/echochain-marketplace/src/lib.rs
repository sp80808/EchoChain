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

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_royalty_distribution::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Currency type for marketplace transactions
        #[pallet::constant]
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Item listed on marketplace [item_id, seller, price]
        ItemListed(u64, T::AccountId, BalanceOf<T>),
        /// Item purchased from marketplace [item_id, buyer, seller, price]
        ItemPurchased(u64, T::AccountId, T::AccountId, BalanceOf<T>),
        /// Item expired and removed from marketplace [item_id]
        ItemExpired(u64),
    }

    /// Storage for marketplace items
    #[pallet::storage]
    #[pallet::getter(fn marketplace_items)]
    pub type MarketplaceItems<T> = StorageMap<_, Blake2_128Concat, u64, ItemInfo<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Item information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct ItemInfo<AccountId, Balance, BlockNumber> {
        seller: AccountId,
        price: Balance,
        description: Vec<u8>,
        royalty_recipients: Option<(AccountId, AccountId, AccountId)>, // (creator, contributor, liquidity_pool)
        listing_block: BlockNumber,
        expiration_block: Option<BlockNumber>,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// List an item for sale on the marketplace
        #[pallet::weight(10_000)]
        pub fn list_item(
            origin: OriginFor<T>,
            item_id: u64,
            price: BalanceOf<T>,
            description: Vec<u8>,
            royalty_recipients: Option<(T::AccountId, T::AccountId, T::AccountId)>,
            duration_blocks: Option<T::BlockNumber>
        ) -> DispatchResult {
            let seller = ensure_signed(origin)?;
            
            ensure!(
                !MarketplaceItems::<T>::contains_key(item_id),
                Error::<T>::ItemAlreadyExists
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            let expiration_block = duration_blocks.map(|duration| current_block.checked_add(&duration).unwrap_or(current_block));

            let item_info = ItemInfo {
                seller: seller.clone(),
                price,
                description,
                royalty_recipients,
                listing_block: current_block,
                expiration_block,
            };
            MarketplaceItems::<T>::insert(item_id, item_info);
            Self::deposit_event(Event::ItemListed(item_id, seller, price));
            Ok(())
        }

        /// List multiple items for sale on the marketplace in a batch to optimize gas costs
        #[pallet::weight(15_000)]
        pub fn list_batch_items(
            origin: OriginFor<T>,
            items: Vec<(u64, BalanceOf<T>, Vec<u8>, Option<(T::AccountId, T::AccountId, T::AccountId)>, Option<T::BlockNumber>)>
        ) -> DispatchResult {
            let seller = ensure_signed(origin)?;
            ensure!(!items.is_empty(), Error::<T>::NoItemsToList);

            let current_block = <frame_system::Pallet<T>>::block_number();

            for (item_id, price, description, royalty_recipients, duration_blocks) in items {
                ensure!(
                    !MarketplaceItems::<T>::contains_key(item_id),
                    Error::<T>::ItemAlreadyExists
                );

                let expiration_block = duration_blocks.map(|duration| current_block.checked_add(&duration).unwrap_or(current_block));

                let item_info = ItemInfo {
                    seller: seller.clone(),
                    price,
                    description,
                    royalty_recipients,
                    listing_block: current_block,
                    expiration_block,
                };
                MarketplaceItems::<T>::insert(item_id, item_info);
                Self::deposit_event(Event::ItemListed(item_id, seller.clone(), price));
            }
            Ok(())
        }

        /// Purchase an item from the marketplace with royalty distribution
        #[pallet::weight(20_000)]
        pub fn purchase_item(
            origin: OriginFor<T>,
            item_id: u64
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;
            
            let item = MarketplaceItems::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;
            ensure!(
                item.seller != buyer,
                Error::<T>::CannotBuyOwnItem
            );

            // Transfer the full price to the seller initially
            T::Currency::transfer(&buyer, &item.seller, item.price, ExistenceRequirement::KeepAlive)?;

            // If royalty recipients are specified, distribute royalties
            if let Some((creator, contributor, liquidity_pool)) = item.royalty_recipients {
                let royalty_result = pallet_royalty_distribution::Pallet::<T>::distribute_royalties(
                    frame_system::RawOrigin::Signed(buyer.clone()).into(),
                    item.price,
                    creator,
                    contributor,
                    liquidity_pool
                );
                if let Err(e) = royalty_result {
                    // Log the error but continue with the purchase
                    log::warn!("Royalty distribution failed: {:?}", e);
                }
            }

            // Remove the item from the marketplace after purchase
            MarketplaceItems::<T>::remove(item_id);
            Self::deposit_event(Event::ItemPurchased(item_id, buyer, item.seller, item.price));
            Ok(())
        }

        /// Purchase multiple items from the marketplace in a batch to optimize gas costs
        #[pallet::weight(30_000)]
        pub fn purchase_batch_items(
            origin: OriginFor<T>,
            item_ids: Vec<u64>
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;
            ensure!(!item_ids.is_empty(), Error::<T>::NoItemsToPurchase);

            let current_block = <frame_system::Pallet<T>>::block_number();

            for item_id in item_ids {
                let item = MarketplaceItems::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;
                ensure!(
                    item.seller != buyer,
                    Error::<T>::CannotBuyOwnItem
                );
                // Check if item has expired
                if let Some(expiration) = item.expiration_block {
                    ensure!(
                        current_block <= expiration,
                        Error::<T>::ItemExpired
                    );
                }

                // Transfer the full price to the seller initially
                T::Currency::transfer(&buyer, &item.seller, item.price, ExistenceRequirement::KeepAlive)?;

                // If royalty recipients are specified, distribute royalties
                if let Some((creator, contributor, liquidity_pool)) = item.royalty_recipients {
                    let royalty_result = pallet_royalty_distribution::Pallet::<T>::distribute_royalties(
                        frame_system::RawOrigin::Signed(buyer.clone()).into(),
                        item.price,
                        creator,
                        contributor,
                        liquidity_pool
                    );
                    if let Err(e) = royalty_result {
                        // Log the error but continue with the purchase
                        log::warn!("Royalty distribution failed for item {}: {:?}", item_id, e);
                    }
                }

                // Remove the item from the marketplace after purchase
                MarketplaceItems::<T>::remove(item_id);
                Self::deposit_event(Event::ItemPurchased(item_id, buyer.clone(), item.seller, item.price));
            }
            Ok(())
        }

        /// Remove expired items from the marketplace
        #[pallet::weight(10_000)]
        pub fn cleanup_expired_items(
            origin: OriginFor<T>,
            max_items_to_check: u32
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let current_block = <frame_system::Pallet<T>>::block_number();
            let mut checked_items = 0;

            // Use a vector to collect keys to avoid mutating while iterating
            let mut expired_items = Vec::new();
            for (item_id, item) in MarketplaceItems::<T>::iter() {
                if checked_items >= max_items_to_check {
                    break;
                }
                if let Some(expiration) = item.expiration_block {
                    if current_block > expiration {
                        expired_items.push(item_id);
                    }
                }
                checked_items += 1;
            }

            // Remove expired items
            for item_id in expired_items {
                MarketplaceItems::<T>::remove(item_id);
                Self::deposit_event(Event::ItemExpired(item_id));
            }
            Ok(())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Item already exists with the given ID
        ItemAlreadyExists,
        /// Item not found in the marketplace
        ItemNotFound,
        /// Cannot buy own item
        CannotBuyOwnItem,
        /// No items provided for listing
        NoItemsToList,
        /// No items provided for purchase
        NoItemsToPurchase,
        /// Item has expired and cannot be purchased
        ItemExpired,
    }

    pub trait MarketplaceInterface<AccountId> {
        fn post_job(who: AccountId, job_id: u64, job_details: Vec<u8>) -> DispatchResult;
    }

    impl<T: Config> MarketplaceInterface<T::AccountId> for Pallet<T> {
        fn post_job(who: T::AccountId, job_id: u64, job_details: Vec<u8>) -> DispatchResult {
            // This is a stub. The actual implementation would post a job to the marketplace.
            Pallet::<T>::deposit_event(Event::ComputeJobPosted(job_id, who, job_details));
            Ok(())
        }
    }
}
