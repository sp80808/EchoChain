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
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_sample(origin: OriginFor<T>, ipfs_cid: Vec<u8>, metadata_ipfs_cid: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
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

        #[pallet::weight(10_000)]
        pub fn update_sample_status(origin: OriginFor<T>, sample_id: u64, status: SampleStatus) -> DispatchResult {
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
}
