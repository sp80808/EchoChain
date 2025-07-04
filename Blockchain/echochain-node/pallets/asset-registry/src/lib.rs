#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::types::*;
    use frame_support::{pallet_prelude::*, BoundedVec};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    // Query helper: Get all assets for an owner
    impl<T: Config> Pallet<T> {
        pub fn get_assets_by_owner(owner: &T::AccountId) -> Vec<T::Hash> {
            AssetsByOwner::<T>::get(owner).into_inner()
        }

        pub fn get_asset(asset_id: &T::Hash) -> Option<RegisteredAsset<T::AccountId, T::BlockNumber, T::Hash, T::MaxIpfsCidLen, T::MaxTagLen, T::MaxTags>> {
            Assets::<T>::get(asset_id)
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MaxIpfsCidLen: Get<u32>;
        type MaxTagLen: Get<u32>;
        type MaxTags: Get<u32>;
        type MaxAssetsPerOwner: Get<u32>;
    }

    #[pallet::storage]
    #[pallet::getter(fn assets)]
    pub type Assets<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        RegisteredAsset<T::AccountId, T::BlockNumber, T::Hash, T::MaxIpfsCidLen, T::MaxTagLen, T::MaxTags>,
        OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn assets_by_owner)]
    pub type AssetsByOwner<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<T::Hash, T::MaxAssetsPerOwner>,
        ValueQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AssetRegistered {
            asset_id: T::Hash,
            owner: T::AccountId,
        },
        AssetDeregistered {
            asset_id: T::Hash,
            owner: T::AccountId,
        },
        AssetUpdated {
            asset_id: T::Hash,
            owner: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        AssetLimitReached,
        DuplicateAsset,
        InvalidInput,
        AssetNotFound,
        NotAssetOwner,
        NoChange,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_asset(
            origin: OriginFor<T>,
            ipfs_cid: BoundedVec<u8, T::MaxIpfsCidLen>,
            tags: BoundedVec<BoundedVec<u8, T::MaxTagLen>, T::MaxTags>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let now = <frame_system::Pallet<T>>::block_number();
            let asset_id = T::Hashing::hash_of(&(&who, &ipfs_cid, &tags, now));

            ensure!(!Assets::<T>::contains_key(&asset_id), Error::<T>::DuplicateAsset);

            let mut owned = AssetsByOwner::<T>::get(&who);
            ensure!((owned.len() as u32) < T::MaxAssetsPerOwner::get(), Error::<T>::AssetLimitReached);

            let asset = RegisteredAsset {
                asset_id,
                owner: who.clone(),
                registered_at: now,
                ipfs_cid: ipfs_cid.clone(),
                tags: tags.clone(),
            };
            Assets::<T>::insert(&asset_id, asset);
            owned.try_push(asset_id).map_err(|_| Error::<T>::AssetLimitReached)?;
            AssetsByOwner::<T>::insert(&who, owned);
            Self::deposit_event(Event::AssetRegistered { asset_id, owner: who });
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn deregister_asset(
            origin: OriginFor<T>,
            asset_id: T::Hash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Ensure asset exists
            let asset = Assets::<T>::get(&asset_id).ok_or(Error::<T>::AssetNotFound)?;
            // Ensure caller is the owner
            ensure!(asset.owner == who, Error::<T>::NotAssetOwner);
            // Remove asset from Assets
            Assets::<T>::remove(&asset_id);
            // Remove asset_id from owner's list
            let mut owned = AssetsByOwner::<T>::get(&who);
            if let Some(pos) = owned.iter().position(|id| id == &asset_id) {
                owned.remove(pos);
                AssetsByOwner::<T>::insert(&who, owned);
            }
            Self::deposit_event(Event::AssetDeregistered { asset_id, owner: who });
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn update_asset(
            origin: OriginFor<T>,
            asset_id: T::Hash,
            new_ipfs_cid: Option<BoundedVec<u8, T::MaxIpfsCidLen>>,
            new_tags: Option<BoundedVec<BoundedVec<u8, T::MaxTagLen>, T::MaxTags>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Assets::<T>::try_mutate_exists(&asset_id, |maybe_asset| {
                let asset = maybe_asset.as_mut().ok_or(Error::<T>::AssetNotFound)?;
                ensure!(asset.owner == who, Error::<T>::NotAssetOwner);
                let mut changed = false;
                if let Some(cid) = new_ipfs_cid {
                    if asset.ipfs_cid != cid {
                        asset.ipfs_cid = cid;
                        changed = true;
                    }
                }
                if let Some(tags) = new_tags {
                    if asset.tags != tags {
                        asset.tags = tags;
                        changed = true;
                    }
                }
                ensure!(changed, Error::<T>::NoChange);
                Self::deposit_event(Event::AssetUpdated { asset_id, owner: who });
                Ok(())
            })
        }
    }
        }
    }
}
