#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::{Currency, ExistenceRequirement, Get}};
    use frame_system::pallet_prelude::*;
    use crate::Config;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
        type AssetRegistry: AssetRegistryProvider<Self::AccountId>;
        type EpochDuration: Get<Self::BlockNumber>;
    }

    pub trait AssetRegistryProvider<AccountId> {
        fn asset_count(who: &AccountId) -> u32;
    }

    #[pallet::storage]
    #[pallet::getter(fn epoch_reward_amount)]
    pub type EpochRewardAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn minimum_assets_for_reward)]
    pub type MinimumAssetsForReward<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_reward_block)]
    pub type LastRewardBlock<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RewardsDistributed { epoch: T::BlockNumber },
        RewardParametersUpdated { amount: BalanceOf<T>, min_assets: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotRoot,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight {
            if (n % T::EpochDuration::get()).is_zero() {
                let min_assets = MinimumAssetsForReward::<T>::get();
                let reward_amount = EpochRewardAmount::<T>::get();
                let mut rewarded = 0u32;
                // Iterate all accounts (in production, use an efficient index or offchain worker)
                // Here, we assume a small devnet for PoC
                for (who, _) in <frame_system::Account<T>>::iter() {
                    if T::AssetRegistry::asset_count(&who) >= min_assets {
                        let _ = T::Currency::deposit_creating(&who, reward_amount);
                        rewarded += 1;
                    }
                }
                LastRewardBlock::<T>::put(n);
                Self::deposit_event(Event::RewardsDistributed { epoch: n });
            }
            0
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn set_reward_parameters(
            origin: OriginFor<T>,
            reward_amount: BalanceOf<T>,
            min_assets: u32
        ) -> DispatchResult {
            ensure_root(origin)?;
            EpochRewardAmount::<T>::put(reward_amount);
            MinimumAssetsForReward::<T>::put(min_assets);
            Self::deposit_event(Event::RewardParametersUpdated { amount: reward_amount, min_assets });
            Ok(())
        }
    }
}
