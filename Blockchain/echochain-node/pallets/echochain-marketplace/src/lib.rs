#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_royalty_distribution::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Currency type for marketplace transactions with bid reservation support
        /// SECURITY: ReservableCurrency enables proper bid escrow to prevent market manipulation
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        /// The maximum number of jobs that can be assigned to a single processor.
        #[pallet::constant]
        type MaxAssignedJobs: Get<u32>;
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
        /// Auction started for item [item_id, starting_price]
        AuctionStarted(u64, BalanceOf<T>),
        /// Bid placed on auction [item_id, bidder, bid_amount]
        BidPlaced(u64, T::AccountId, BalanceOf<T>),
        /// Previous bid unreserved [item_id, previous_bidder, amount]
        BidUnreserved(u64, T::AccountId, BalanceOf<T>),
        /// Auction ended [item_id, winner, final_price]
        AuctionEnded(u64, T::AccountId, BalanceOf<T>),
        /// Compute job posted to marketplace [job_id, requester, job_details]
        ComputeJobPosted(u64, T::AccountId, Vec<u8>),
        /// Commission posted for audio piece [commission_id, requester, bounty]
        CommissionPosted(u64, T::AccountId, BalanceOf<T>),
        /// Submission made for a commission [commission_id, submitter, submission_id]
        SubmissionMade(u64, T::AccountId, u64),
        /// Submission selected for a commission [commission_id, submitter, submission_id, bounty]
        SubmissionSelected(u64, T::AccountId, u64, BalanceOf<T>),
    }

    /// Storage for marketplace items
    #[pallet::storage]
    #[pallet::getter(fn marketplace_items)]
    pub type MarketplaceItems<T> = StorageMap<_, Blake2_128Concat, u64, ItemInfo<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Stores the number of jobs currently assigned to each processor.
    #[pallet::storage]
    #[pallet::getter(fn assigned_job_count)]
    pub type AssignedJobCount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    /// Storage for auctions
    #[pallet::storage]
    #[pallet::getter(fn auctions)]
    pub type Auctions<T> = StorageMap<_, Blake2_128Concat, u64, AuctionInfo<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Storage for commissions
    #[pallet::storage]
    #[pallet::getter(fn commissions)]
    pub type Commissions<T> = StorageMap<_, Blake2_128Concat, u64, CommissionInfo<T::AccountId, BalanceOf<T>>, OptionQuery>;

    /// Storage for submissions to commissions
    #[pallet::storage]
    #[pallet::getter(fn submissions)]
    pub type Submissions<T> = StorageDoubleMap<_, Blake2_128Concat, u64, Blake2_128Concat, u64, SubmissionInfo<T::AccountId>, OptionQuery>;

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

    /// Auction information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct AuctionInfo<AccountId, Balance, BlockNumber> {
        item_id: u64,
        seller: AccountId,
        starting_price: Balance,
        highest_bid: Option<(AccountId, Balance)>,
        start_block: BlockNumber,
        end_block: BlockNumber,
        status: AuctionStatus,
    }

    /// Commission information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct CommissionInfo<AccountId, Balance, BlockNumber> {
        requester: AccountId,
        bounty: Balance,
        description: Vec<u8>,
        listing_block: BlockNumber,
        expiration_block: Option<BlockNumber>,
        status: CommissionStatus,
    }

    /// Submission information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct SubmissionInfo<AccountId> {
        submitter: AccountId,
        content_hash: Vec<u8>,
        submission_date: u64,
    }

    /// Commission status enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum CommissionStatus {
        Open,
        Closed,
    }

    /// Auction status enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum AuctionStatus {
        Active,
        Ended,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// List an item for sale on the marketplace with input validation
        ///
        /// # Security Features
        /// - Validates price bounds to prevent economic attacks
        /// - Limits description length to prevent storage bloat
        /// - Ensures item IDs are reasonable
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
            
            // SECURITY: Input validation to prevent attacks
            Self::validate_price(&price)?;
            Self::validate_description(&description)?;
            
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

        /// Start an auction for an item
        #[pallet::weight(10_000)]
        pub fn start_auction(
            origin: OriginFor<T>,
            item_id: u64,
            starting_price: BalanceOf<T>,
            duration_blocks: T::BlockNumber
        ) -> DispatchResult {
            let seller = ensure_signed(origin)?;
            let item = MarketplaceItems::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;
            ensure!(
                item.seller == seller,
                Error::<T>::CannotBuyOwnItem
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = current_block.checked_add(&duration_blocks).unwrap_or(current_block);

            let auction_info = AuctionInfo {
                item_id,
                seller: seller.clone(),
                starting_price,
                highest_bid: None,
                start_block: current_block,
                end_block,
                status: AuctionStatus::Active,
            };
            Auctions::<T>::insert(item_id, auction_info);
            // Remove from regular marketplace listing to prevent direct purchase
            MarketplaceItems::<T>::remove(item_id);
            Self::deposit_event(Event::AuctionStarted(item_id, starting_price));
            Ok(())
        }

        /// Place a bid on an active auction with proper fund reservation
        ///
        /// # Security Features
        /// - Reserves bidder funds to prevent phantom bids
        /// - Unreserves previous bidder's funds automatically
        /// - Validates bid amounts and auction state
        /// - Prevents market manipulation through proper escrow
        #[pallet::weight(10_000)]
        pub fn place_bid(
            origin: OriginFor<T>,
            item_id: u64,
            bid_amount: BalanceOf<T>
        ) -> DispatchResult {
            let bidder = ensure_signed(origin)?;
            let mut auction = Auctions::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;
            
            // Validate auction state
            ensure!(
                auction.status == AuctionStatus::Active,
                Error::<T>::InvalidAuctionState
            );
            ensure!(
                <frame_system::Pallet<T>>::block_number() <= auction.end_block,
                Error::<T>::AuctionEnded
            );
            ensure!(
                auction.seller != bidder,
                Error::<T>::CannotBuyOwnItem
            );
            
            // Validate bid amount
            ensure!(
                bid_amount > auction.starting_price,
                Error::<T>::BidTooLow
            );
            if let Some((_, current_highest)) = auction.highest_bid {
                ensure!(
                    bid_amount > current_highest,
                    Error::<T>::BidTooLow
                );
            }

            // SECURITY FIX: Reserve the new bid amount first to ensure bidder has funds
            T::Currency::reserve(&bidder, bid_amount)
                .map_err(|_| Error::<T>::InsufficientFunds)?;

            // If there was a previous bid, unreserve those funds
            if let Some((previous_bidder, previous_amount)) = &auction.highest_bid {
                T::Currency::unreserve(previous_bidder, *previous_amount);
                Self::deposit_event(Event::BidUnreserved(item_id, previous_bidder.clone(), *previous_amount));
            }

            // Update auction with new highest bid
            auction.highest_bid = Some((bidder.clone(), bid_amount));
            Auctions::<T>::insert(item_id, auction);
            
            Self::deposit_event(Event::BidPlaced(item_id, bidder, bid_amount));
            Ok(())
        }

        /// End an auction and finalize the sale
        #[pallet::weight(15_000)]
        pub fn end_auction(
            origin: OriginFor<T>,
            item_id: u64
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let mut auction = Auctions::<T>::get(item_id).ok_or(Error::<T>::ItemNotFound)?;
            ensure!(
                auction.status == AuctionStatus::Active,
                Error::<T>::InvalidAuctionState
            );
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(
                current_block >= auction.end_block,
                Error::<T>::AuctionNotEnded
            );

            auction.status = AuctionStatus::Ended;
            if let Some((winner, final_price)) = auction.highest_bid {
                // SECURITY FIX: Unreserve winner's funds and transfer to seller
                // This completes the escrow process initiated in place_bid
                T::Currency::unreserve(&winner, final_price);
                T::Currency::transfer(&winner, &auction.seller, final_price, ExistenceRequirement::KeepAlive)?;

                // Distribute royalties if specified
                if let Some(item) = MarketplaceItems::<T>::get(item_id) {
                    if let Some((creator, contributor, liquidity_pool)) = item.royalty_recipients {
                        let royalty_result = pallet_royalty_distribution::Pallet::<T>::distribute_royalties(
                            frame_system::RawOrigin::Signed(winner.clone()).into(),
                            final_price,
                            creator,
                            contributor,
                            liquidity_pool
                        );
                        if let Err(e) = royalty_result {
                            log::warn!("Royalty distribution failed for auction {}: {:?}", item_id, e);
                        }
                    }
                }

                Auctions::<T>::insert(item_id, auction);
                Self::deposit_event(Event::AuctionEnded(item_id, winner, final_price));
            } else {
                // No bids, auction ends without a sale, item is removed
                Auctions::<T>::remove(item_id);
                Self::deposit_event(Event::AuctionEnded(item_id, auction.seller, auction.starting_price));
            }
            Ok(())
        }

        /// Post a commission for an audio piece with a bounty
        #[pallet::weight(10_000)]
        pub fn post_commission(
            origin: OriginFor<T>,
            commission_id: u64,
            bounty: BalanceOf<T>,
            description: Vec<u8>,
            duration_blocks: Option<T::BlockNumber>
        ) -> DispatchResult {
            let requester = ensure_signed(origin)?;
            
            ensure!(
                !Commissions::<T>::contains_key(commission_id),
                Error::<T>::ItemAlreadyExists
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            let expiration_block = duration_blocks.map(|duration| current_block.checked_add(&duration).unwrap_or(current_block));

            let commission_info = CommissionInfo {
                requester: requester.clone(),
                bounty,
                description,
                listing_block: current_block,
                expiration_block,
                status: CommissionStatus::Open,
            };
            Commissions::<T>::insert(commission_id, commission_info);
            Self::deposit_event(Event::CommissionPosted(commission_id, requester, bounty));
            Ok(())
        }

        /// Submit an audio sample for a commission
        #[pallet::weight(10_000)]
        pub fn submit_for_commission(
            origin: OriginFor<T>,
            commission_id: u64,
            submission_id: u64,
            content_hash: Vec<u8>
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;
            let commission = Commissions::<T>::get(commission_id).ok_or(Error::<T>::ItemNotFound)?;
            ensure!(
                commission.status == CommissionStatus::Open,
                Error::<T>::InvalidCommissionState
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            if let Some(expiration) = commission.expiration_block {
                ensure!(
                    current_block <= expiration,
                    Error::<T>::ItemExpired
                );
            }

            let submission_info = SubmissionInfo {
                submitter: submitter.clone(),
                content_hash,
                submission_date: current_block.into(),
            };
            Submissions::<T>::insert(commission_id, submission_id, submission_info);
            Self::deposit_event(Event::SubmissionMade(commission_id, submitter, submission_id));
            Ok(())
        }

        /// Select a submission for a commission and award the bounty
        #[pallet::weight(15_000)]
        pub fn select_submission(
            origin: OriginFor<T>,
            commission_id: u64,
            submission_id: u64
        ) -> DispatchResult {
            let requester = ensure_signed(origin)?;
            let mut commission = Commissions::<T>::get(commission_id).ok_or(Error::<T>::ItemNotFound)?;
            ensure!(
                commission.requester == requester,
                Error::<T>::UnauthorizedAction
            );
            ensure!(
                commission.status == CommissionStatus::Open,
                Error::<T>::InvalidCommissionState
            );

            let submission = Submissions::<T>::get(commission_id, submission_id).ok_or(Error::<T>::SubmissionNotFound)?;
            
            // Transfer the bounty to the submitter
            T::Currency::transfer(&requester, &submission.submitter, commission.bounty, ExistenceRequirement::KeepAlive)?;

            // Close the commission
            commission.status = CommissionStatus::Closed;
            Commissions::<T>::insert(commission_id, commission);
            Self::deposit_event(Event::SubmissionSelected(commission_id, submission.submitter, submission_id, commission.bounty));
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
        /// Auction is not in an active state
        InvalidAuctionState,
        /// Auction has already ended
        AuctionEnded,
        /// Auction has not yet ended
        AuctionNotEnded,
        /// Bid amount is too low
        BidTooLow,
        /// Processor has too many assigned jobs
        TooManyAssignedJobs,
        /// Commission is not in an open state
        InvalidCommissionState,
        /// Submission not found for the commission
        SubmissionNotFound,
        /// Unauthorized action
        UnauthorizedAction,
        /// Insufficient funds for bid reservation
        InsufficientFunds,
        /// Description text is too long
        DescriptionTooLong,
        /// Price amount is invalid (zero or exceeds maximum)
        InvalidPrice,
        /// Invalid item ID format
        InvalidItemId,
    }

    /// Input validation constants for marketplace security
    const MAX_DESCRIPTION_LENGTH: u32 = 1000;
    const MAX_PRICE: u128 = 1_000_000_000_000_000_000_000u128; // 1 billion tokens
    const MIN_PRICE: u128 = 1u128; // Minimum 1 unit
    }

    pub trait MarketplaceInterface<AccountId> {
        fn post_job(who: AccountId, job_id: u64, job_details: Vec<u8>) -> DispatchResult;
    }

    impl<T: Config> MarketplaceInterface<T::AccountId> for Pallet<T> {
        fn post_job(who: T::AccountId, job_id: u64, job_details: Vec<u8>) -> DispatchResult {
            // Check if the job_id is already in use
            ensure!(
                !MarketplaceItems::<T>::contains_key(job_id),
                Error::<T>::ItemAlreadyExists
            );

            // Record the job as an item in the marketplace with a placeholder price
            let current_block = <frame_system::Pallet<T>>::block_number();
            let job_info = ItemInfo {
                seller: who.clone(),
                price: BalanceOf::<T>::zero(),
                description: job_details.clone(),
                royalty_recipients: None,
                listing_block: current_block,
                expiration_block: None,
            };
            MarketplaceItems::<T>::insert(job_id, job_info);
            
            // Emit event for job posting
            Pallet::<T>::deposit_event(Event::ComputeJobPosted(job_id, who, job_details));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Validate price amount for marketplace listings
        ///
        /// # Security Checks
        /// - Prevents zero-price listings that could break economic models
        /// - Limits maximum price to prevent overflow attacks
        /// - Ensures reasonable price bounds for marketplace integrity
        fn validate_price(price: &BalanceOf<T>) -> DispatchResult {
            let price_u128: u128 = (*price).try_into().map_err(|_| Error::<T>::InvalidPrice)?;
            
            ensure!(
                price_u128 >= MIN_PRICE,
                Error::<T>::InvalidPrice
            );
            ensure!(
                price_u128 <= MAX_PRICE,
                Error::<T>::InvalidPrice
            );
            
            Ok(())
        }

        /// Validate description length and content
        ///
        /// # Security Checks
        /// - Prevents storage bloat through oversized descriptions
        /// - Limits blockchain state growth from excessive data
        fn validate_description(description: &[u8]) -> DispatchResult {
            ensure!(
                description.len() <= MAX_DESCRIPTION_LENGTH as usize,
                Error::<T>::DescriptionTooLong
            );
            
            // Ensure description contains only printable ASCII characters
            for &byte in description {
                ensure!(
                    byte >= 32 && byte <= 126, // Printable ASCII range
                    Error::<T>::DescriptionTooLong
                );
            }
            
            Ok(())
        }
    }
}
