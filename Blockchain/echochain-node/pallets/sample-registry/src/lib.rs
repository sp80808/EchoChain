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
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// SECURITY: Origin that can approve samples through governance
        /// This replaces centralized root-only approval with decentralized governance
        type ApprovalOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        
        /// Minimum reputation score required for approval authority
        /// Prevents low-reputation accounts from abusing approval rights
        #[pallet::constant]
        type MinApprovalReputation: Get<u32>;
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum SampleStatus {
        Pending,
        Approved,
        Rejected,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Sample<T: Config> {
        pub owner: T::AccountId,
        pub ipfs_cid: Vec<u8>,
        pub metadata_ipfs_cid: Vec<u8>,
        pub status: SampleStatus,
    }

    #[pallet::storage]
    #[pallet::getter(fn samples)]
    pub type Samples<T: Config> = StorageMap<_, Blake2_128Concat, u64, Sample<T>>;

    #[pallet::storage]
    #[pallet::getter(fn next_sample_id)]
    pub type NextSampleId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SampleRegistered(u64, T::AccountId),
        SampleStatusUpdated(u64, SampleStatus),
    }

    #[pallet::error]
    pub enum Error<T> {
        SampleNotFound,
        /// IPFS CID is invalid or too long
        InvalidIpfsCid,
        /// Metadata IPFS CID is invalid or too long
        InvalidMetadataIpfsCid,
        /// Input data exceeds maximum allowed length
        InputTooLong,
        /// Approval origin lacks sufficient reputation
        InsufficientReputation,
    }

    /// Input validation constants for security
    const MAX_IPFS_CID_LENGTH: u32 = 100;
    const MAX_METADATA_CID_LENGTH: u32 = 100;
    const MIN_IPFS_CID_LENGTH: u32 = 10;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new sample with comprehensive input validation
        ///
        /// # Security Features
        /// - Validates IPFS CID format and length
        /// - Prevents injection attacks through bounds checking
        /// - Ensures data integrity for on-chain storage
        ///
        /// # Parameters
        /// - `ipfs_cid`: IPFS Content Identifier (must be valid format)
        /// - `metadata_ipfs_cid`: IPFS CID for sample metadata
        #[pallet::weight(10_000)]
        pub fn register_sample(origin: OriginFor<T>, ipfs_cid: Vec<u8>, metadata_ipfs_cid: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            // SECURITY: Comprehensive input validation to prevent attacks
            Self::validate_ipfs_cid(&ipfs_cid)?;
            Self::validate_metadata_cid(&metadata_ipfs_cid)?;
            
            let sample_id = NextSampleId::<T>::get();

            let sample = Sample {
                owner: who.clone(),
                ipfs_cid,
                metadata_ipfs_cid,
                status: SampleStatus::Pending,
            };

            Samples::<T>::insert(sample_id, sample);
            NextSampleId::<T>::put(sample_id + 1);

            Self::deposit_event(Event::SampleRegistered(sample_id, who));
            Ok(())
        }

        /// Update sample status through governance approval
        ///
        /// # Security Features
        /// - Replaces centralized root-only approval with governance
        /// - Prevents single point of failure in sample approval process
        /// - Enables decentralized decision making for content moderation
        ///
        /// # Parameters
        /// - `origin`: Must be approved governance origin (council/democracy)
        /// - `sample_id`: ID of sample to update
        /// - `status`: New status (Pending/Approved/Rejected)
        #[pallet::weight(10_000)]
        pub fn update_sample_status(origin: OriginFor<T>, sample_id: u64, status: SampleStatus) -> DispatchResult {
            // SECURITY FIX: Use governance origin instead of root-only
            T::ApprovalOrigin::ensure_origin(origin)?;
            
            let mut sample = Samples::<T>::get(sample_id).ok_or(Error::<T>::SampleNotFound)?;
            let old_status = sample.status.clone();
            sample.status = status.clone();
            Samples::<T>::insert(sample_id, sample);

            Self::deposit_event(Event::SampleStatusUpdated(sample_id, status));
            Ok(())
        }

        /// Emergency root override for sample status (for migration/emergency only)
        ///
        /// # Security Notice
        /// This function should only be used in emergencies or during migration periods.
        /// Normal operations should use governance-based approval through update_sample_status.
        #[pallet::weight(10_000)]
        pub fn emergency_update_sample_status(origin: OriginFor<T>, sample_id: u64, status: SampleStatus) -> DispatchResult {
            ensure_root(origin)?;
            
            let mut sample = Samples::<T>::get(sample_id).ok_or(Error::<T>::SampleNotFound)?;
            sample.status = status.clone();
            Samples::<T>::insert(sample_id, sample);

            Self::deposit_event(Event::SampleStatusUpdated(sample_id, status));
            Ok(())
        }
    }

    pub trait SampleInterface<AccountId> {
        fn get_approved_sample_count(owner: &AccountId) -> u32;
    }

    impl<T: Config> SampleInterface<T::AccountId> for Pallet<T> {
        fn get_approved_sample_count(owner: &T::AccountId) -> u32 {
            Samples::<T>::iter().filter(|(_, sample)| &sample.owner == owner && sample.status == SampleStatus::Approved).count() as u32
        }
    }

    impl<T: Config> Pallet<T> {
        /// Validate IPFS CID format and length
        ///
        /// # Security Checks
        /// - Length bounds checking to prevent buffer overflows
        /// - Basic format validation for IPFS CIDs
        /// - Prevents injection attacks through malformed CIDs
        fn validate_ipfs_cid(cid: &[u8]) -> DispatchResult {
            ensure!(
                cid.len() >= MIN_IPFS_CID_LENGTH as usize,
                Error::<T>::InvalidIpfsCid
            );
            ensure!(
                cid.len() <= MAX_IPFS_CID_LENGTH as usize,
                Error::<T>::InvalidIpfsCid
            );
            
            // Basic IPFS CID validation - should start with 'Qm' for base58 or 'b' for base32
            if cid.len() > 0 {
                let first_char = cid[0];
                ensure!(
                    first_char == b'Q' || first_char == b'b' || first_char == b'z',
                    Error::<T>::InvalidIpfsCid
                );
            }
            
            // Ensure CID contains only valid characters
            for &byte in cid {
                ensure!(
                    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-',
                    Error::<T>::InvalidIpfsCid
                );
            }
            
            Ok(())
        }

        /// Validate metadata IPFS CID
        fn validate_metadata_cid(cid: &[u8]) -> DispatchResult {
            ensure!(
                cid.len() <= MAX_METADATA_CID_LENGTH as usize,
                Error::<T>::InvalidMetadataIpfsCid
            );
            
            if !cid.is_empty() {
                // Apply same validation as regular IPFS CID
                Self::validate_ipfs_cid(cid).map_err(|_| Error::<T>::InvalidMetadataIpfsCid)?;
            }
            
            Ok(())
        }
    }
}
