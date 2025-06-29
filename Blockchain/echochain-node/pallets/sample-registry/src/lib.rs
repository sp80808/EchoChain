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

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum SampleStatus {
		Pending,
		Approved,
		Rejected,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct SampleMetadata<T: Config> {
		pub owner: T::AccountId,
		pub ipfs_cid: Vec<u8>,
		pub metadata_ipfs_cid: Vec<u8>,
		pub status: SampleStatus,
		pub created_at: T::BlockNumber,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The maximum length of an IPFS CID.
		#[pallet::constant]
		type MaxIpfsCidLength: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn samples)]
	pub(super) type Samples<T: Config> = StorageMap<
		_, 
		Blake2_128Concat,
		u32, // Sample ID
		SampleMetadata<T>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn next_sample_id)]
	pub(super) type NextSampleId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new sample has been registered.
		SampleRegistered { sample_id: u32, owner: T::AccountId, ipfs_cid: Vec<u8> },
		/// A sample's status has been updated.
		SampleStatusUpdated { sample_id: u32, new_status: SampleStatus },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The IPFS CID is too long.
		IpfsCidTooLong,
		/// The sample does not exist.
		SampleNotFound,
		/// Only the owner or a privileged origin can update the sample status.
		Unauthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new music sample.
		/// This extrinsic can only be called by a trusted origin (e.g., the backend oracle).
		#[pallet::weight(10_000 + T::DbWeight::writes(1).ref_time())]
		pub fn register_sample(
			origin: OriginFor<T>,
			ipfs_cid: Vec<u8>,
			metadata_ipfs_cid: Vec<u8>,
		) -> DispatchResult {
			// Ensure that this call can only be made by a trusted origin (e.g., Root or a designated Oracle).
			// For now, we'll allow Signed, but this should be restricted in a production environment.
			let sender = ensure_signed(origin)?;

			ensure!(
				ipfs_cid.len() <= T::MaxIpfsCidLength::get() as usize,
				Error::<T>::IpfsCidTooLong
			);
			ensure!(
				metadata_ipfs_cid.len() <= T::MaxIpfsCidLength::get() as usize,
				Error::<T>::IpfsCidTooLong
			);

			let sample_id = NextSampleId::<T>::get();
			let new_sample = SampleMetadata {
				owner: sender.clone(),
				ipfs_cid,
				metadata_ipfs_cid,
				status: SampleStatus::Pending,
				created_at: <frame_system::Pallet<T>>::block_number(),
			};

			<Samples<T>>::insert(sample_id, new_sample);
			<NextSampleId<T>>::put(sample_id + 1);

			Self::deposit_event(Event::SampleRegistered { sample_id, owner: sender, ipfs_cid: new_sample.ipfs_cid });
			Ok(())
		}

		/// Update the status of a sample (e.g., from Pending to Approved/Rejected).
		/// This extrinsic should only be callable by a privileged origin (e.g., Root or an Oracle).
		#[pallet::weight(10_000 + T::DbWeight::writes(1).ref_time())]
		pub fn update_sample_status(
			origin: OriginFor<T>,
			sample_id: u32,
			new_status: SampleStatus,
		) -> DispatchResult {
			// Only Root can update sample status for now. This will be refined with an Oracle pallet.
			ensure_root(origin)?;

			<Samples<T>>::try_mutate(sample_id, |sample_opt| -> DispatchResult {
				let sample = sample_opt.as_mut().ok_or(Error::<T>::SampleNotFound)?;
				sample.status = new_status.clone();
				Ok(())
			})?;

			Self::deposit_event(Event::SampleStatusUpdated { sample_id, new_status });
			Ok(())
		}
	}
}
