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
        /// Currency type for potential incentives in P2P activities
        #[pallet::constant]
        type Currency: Currency<Self::AccountId>;
        /// Timeout duration for file transfers in blocks
        #[pallet::constant]
        type TransferTimeout: Get<Self::BlockNumber>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// P2P node registered [account]
        P2PNodeRegistered(T::AccountId),
        /// P2P activity reported [account, bytes_uploaded, bytes_downloaded]
        P2PActivityReported(T::AccountId, u64, u64),
        /// Compute job triggered [account, job_id, task_id]
        ComputeJobTriggered(T::AccountId, u64, u32),
        /// File share registered [file_id, owner, file_hash]
        FileShareRegistered(u64, T::AccountId, T::Hash),
        /// File transfer initiated [file_id, sender, receiver]
        FileTransferInitiated(u64, T::AccountId, T::AccountId),
        /// File transfer completed [file_id, sender, receiver]
        FileTransferCompleted(u64, T::AccountId, T::AccountId),
        /// File transfer timed out [file_id, sender, receiver]
        FileTransferTimedOut(u64, T::AccountId, T::AccountId),
    }

    /// Storage for registered P2P nodes
    #[pallet::storage]
    #[pallet::getter(fn p2p_nodes)]
    pub type P2PNodes<T> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    /// Storage for file shares
    #[pallet::storage]
    #[pallet::getter(fn file_shares)]
    pub type FileShares<T> = StorageMap<_, Blake2_128Concat, u64, FileShareInfo<T::AccountId, T::Hash>, OptionQuery>;

    /// Storage for active file transfers
    #[pallet::storage]
    #[pallet::getter(fn file_transfers)]
    pub type FileTransfers<T> = StorageDoubleMap<_, Blake2_128Concat, u64, Blake2_128Concat, T::AccountId, FileTransferInfo<T::AccountId>, OptionQuery>;

    /// File share information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct FileShareInfo<AccountId, Hash> {
        owner: AccountId,
        file_hash: Hash,
        description: Vec<u8>,
    }

    /// File transfer information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct FileTransferInfo<AccountId, BlockNumber> {
        sender: AccountId,
        receiver: AccountId,
        status: TransferStatus,
        initiation_block: BlockNumber,
    }

    /// Transfer status enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum TransferStatus {
        Initiated,
        Completed,
        TimedOut,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register as a P2P node
        #[pallet::weight(5_000)]
        pub fn register_p2p_node(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            P2PNodes::<T>::insert(&who, true);
            Self::deposit_event(Event::P2PNodeRegistered(who));
            Ok(())
        }

        /// Report P2P activity for network rewards
        #[pallet::weight(10_000)]
        pub fn report_p2p_activity(origin: OriginFor<T>, bytes_uploaded: u64, bytes_downloaded: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                P2PNodes::<T>::get(&who),
                Error::<T>::NotP2PNode
            );
            T::NetworkRewards::submit_report(who.clone(), bytes_uploaded, bytes_downloaded)?;
            Self::deposit_event(Event::P2PActivityReported(who, bytes_uploaded, bytes_downloaded));
            Ok(())
        }

        /// Trigger a compute job through marketplace and compute pallets
        #[pallet::weight(10_000)]
        pub fn trigger_compute_job(origin: OriginFor<T>, job_id: u64, task_id: u32, job_details: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                P2PNodes::<T>::get(&who),
                Error::<T>::NotP2PNode
            );
            T::EchochainMarketplace::post_job(who.clone(), job_id, job_details)?;
            T::EchochainCompute::create_task(who.clone(), task_id)?;
            Self::deposit_event(Event::ComputeJobTriggered(who, job_id, task_id));
            Ok(())
        }

        /// Register a file for sharing in the P2P network
        #[pallet::weight(8_000)]
        pub fn register_file_share(
            origin: OriginFor<T>,
            file_id: u64,
            file_hash: T::Hash,
            description: Vec<u8>
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            ensure!(
                P2PNodes::<T>::get(&owner),
                Error::<T>::NotP2PNode
            );
            ensure!(
                !FileShares::<T>::contains_key(file_id),
                Error::<T>::FileAlreadyExists
            );

            let file_info = FileShareInfo {
                owner: owner.clone(),
                file_hash,
                description,
            };
            FileShares::<T>::insert(file_id, file_info);
            Self::deposit_event(Event::FileShareRegistered(file_id, owner, file_hash));
            Ok(())
        }

        /// Initiate a file transfer to another P2P node
        #[pallet::weight(10_000)]
        pub fn initiate_file_transfer(
            origin: OriginFor<T>,
            file_id: u64,
            receiver: T::AccountId
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(
                P2PNodes::<T>::get(&sender) && P2PNodes::<T>::get(&receiver),
                Error::<T>::NotP2PNode
            );
            let file_share = FileShares::<T>::get(file_id).ok_or(Error::<T>::FileNotFound)?;
            ensure!(
                file_share.owner == sender,
                Error::<T>::NotFileOwner
            );
            ensure!(
                !FileTransfers::<T>::contains_key(file_id, &receiver),
                Error::<T>::TransferAlreadyInitiated
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            let transfer_info = FileTransferInfo {
                sender: sender.clone(),
                receiver: receiver.clone(),
                status: TransferStatus::Initiated,
                initiation_block: current_block,
            };
            FileTransfers::<T>::insert(file_id, &receiver, transfer_info);
            Self::deposit_event(Event::FileTransferInitiated(file_id, sender, receiver));
            Ok(())
        }

        /// Confirm completion of a file transfer with integrity check
        #[pallet::weight(8_000)]
        pub fn confirm_file_transfer(
            origin: OriginFor<T>,
            file_id: u64,
            received_file_hash: T::Hash
        ) -> DispatchResult {
            let receiver = ensure_signed(origin)?;
            ensure!(
                P2PNodes::<T>::get(&receiver),
                Error::<T>::NotP2PNode
            );
            let mut transfer = FileTransfers::<T>::get(file_id, &receiver).ok_or(Error::<T>::TransferNotFound)?;
            ensure!(
                transfer.receiver == receiver && transfer.status == TransferStatus::Initiated,
                Error::<T>::InvalidTransferState
            );

            // Verify file integrity by comparing received hash with original file hash
            let file_share = FileShares::<T>::get(file_id).ok_or(Error::<T>::FileNotFound)?;
            ensure!(
                file_share.file_hash == received_file_hash,
                Error::<T>::FileIntegrityCheckFailed
            );

            transfer.status = TransferStatus::Completed;
            FileTransfers::<T>::insert(file_id, &receiver, transfer);
            Self::deposit_event(Event::FileTransferCompleted(file_id, transfer.sender, receiver));
            Ok(())
        }

        /// Check for timed out file transfers and mark them as failed
        #[pallet::weight(10_000)]
        pub fn check_transfer_timeouts(
            origin: OriginFor<T>,
            max_transfers_to_check: u32
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let current_block = <frame_system::Pallet<T>>::block_number();
            let timeout_duration = T::TransferTimeout::get();
            let mut checked_transfers = 0;

            // Use a vector to collect keys to avoid mutating while iterating
            let mut timed_out_transfers = Vec::new();
            for (file_id, receiver, transfer) in FileTransfers::<T>::iter() {
                if checked_transfers >= max_transfers_to_check {
                    break;
                }
                if transfer.status == TransferStatus::Initiated {
                    if let Some(timeout_block) = transfer.initiation_block.checked_add(&timeout_duration) {
                        if current_block > timeout_block {
                            timed_out_transfers.push((file_id, receiver, transfer.sender.clone(), transfer.receiver.clone()));
                        }
                    }
                }
                checked_transfers += 1;
            }

            // Mark timed out transfers
            for (file_id, receiver, sender, receiver_acc) in timed_out_transfers {
                if let Some(mut transfer) = FileTransfers::<T>::get(file_id, &receiver) {
                    if transfer.status == TransferStatus::Initiated {
                        transfer.status = TransferStatus::TimedOut;
                        FileTransfers::<T>::insert(file_id, &receiver, transfer);
                        Self::deposit_event(Event::FileTransferTimedOut(file_id, sender, receiver_acc));
                    }
                }
            }
            Ok(())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Account is not a registered P2P node
        NotP2PNode,
        /// File already exists with the given ID
        FileAlreadyExists,
        /// File not found for sharing
        FileNotFound,
        /// Account is not the owner of the file
        NotFileOwner,
        /// File transfer already initiated for this receiver
        TransferAlreadyInitiated,
        /// File transfer not found
        TransferNotFound,
        /// Invalid transfer state for confirmation
        InvalidTransferState,
        /// File integrity check failed during transfer confirmation
        FileIntegrityCheckFailed,
    }
}
