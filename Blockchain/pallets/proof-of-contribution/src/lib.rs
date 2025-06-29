#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch::DispatchResult,
    traits::{Currency, Get}, // Import Get trait
};
use sp_std::prelude::*;
use frame_system::ensure_signed; // Import ensure_signed

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Currency: Currency<<Self as frame_system::Config>::AccountId>;

    /// The amount of tokens rewarded for each unit of contribution.
    type ContributionReward: Get<u128>;
}

decl_storage! {
    trait Store for Module<T: Config> as ProofOfContribution {
        /// Storage for tracking content contributions (e.g., sample uploads, metadata quality)
        pub ContentContributions get(fn content_contributions):
            map hasher(blake2_128_concat) T::AccountId => u128; // Changed to u128 for larger values
        
        /// Storage for tracking network contributions (e.g., storage provided, bandwidth used)
        pub NetworkContributions get(fn network_contributions):
            map hasher(blake2_128_concat) T::AccountId => u128; // Changed to u128 for larger values

        /// Total accumulated contributions that are yet to be claimed as rewards.
        pub TotalUnclaimedRewards get(fn total_unclaimed_rewards): u128;
        
        /// Last block number when rewards were distributed
        pub LastRewardBlock get(fn last_reward_block): T::BlockNumber;
        
        /// Reward distribution interval in blocks
        pub RewardInterval get(fn reward_interval): T::BlockNumber;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        /// A user has contributed content. [who, amount]
        ContentContributed(AccountId, u128),
        /// A user has contributed to the network. [who, amount]
        NetworkContributed(AccountId, u128),
        /// A user has claimed rewards for their contributions. [who, amount]
        RewardsClaimed(AccountId, u128),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        /// User has no contributions to claim rewards.
        NoContributionsToClaim,
        /// Overflow occurred during reward calculation.
        RewardCalculationOverflow,
        /// Insufficient funds in the module's account to pay out rewards.
        InsufficientModuleBalance,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        fn on_initialize(now: T::BlockNumber) -> frame_support::weights::Weight {
            let reward_interval = <RewardInterval>::get();
            let last_reward_block = <LastRewardBlock>::get();
            
            if now >= last_reward_block + reward_interval {
                // Distribute rewards automatically
                if let Err(e) = Self::distribute_rewards() {
                    log::error!("Failed to distribute rewards: {:?}", e);
                }
                <LastRewardBlock>::put(now);
            }
            
            // Return minimal weight since this is a simple check
            <T as frame_system::Config>::DbWeight::get().reads_writes(2, 1)
        }

        /// Record a content contribution for a given account.
        /// This function would typically be called by other pallets or smart contracts
        /// when a user performs an action deemed a "content contribution".
        #[weight = 10_000]
        pub fn record_content_contribution(origin, amount: u128) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            <ContentContributions<T>>::mutate(sender.clone(), |contributions| {
                *contributions = contributions.checked_add(amount).ok_or(Error::<T>::RewardCalculationOverflow)?;
            });
            Self::deposit_event(RawEvent::ContentContributed(sender, amount));
            Ok(())
        }

        /// Record a network contribution for a given account.
        /// This function would typically be called by other pallets or network modules
        /// when a user provides storage, bandwidth, or other network resources.
        #[weight = 10_000]
        pub fn record_network_contribution(origin, amount: u128) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            <NetworkContributions<T>>::mutate(sender.clone(), |contributions| {
                *contributions = contributions.checked_add(amount).ok_or(Error::<T>::RewardCalculationOverflow)?;
            });
            Self::deposit_event(RawEvent::NetworkContributed(sender, amount));
            Ok(())
        }

        /// Allows a user to claim rewards based on their accumulated contributions.
        /// The reward calculation is based on a simple formula:
        /// (ContentContributions + NetworkContributions) * ContributionReward
        #[weight = 10_000]
        pub fn claim_rewards(origin) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let content_contrib = <ContentContributions<T>>::take(&sender);
            let network_contrib = <NetworkContributions<T>>::take(&sender);

            let total_contributions = content_contrib.checked_add(network_contrib)
                .ok_or(Error::<T>::RewardCalculationOverflow)?;

            if total_contributions == 0 {
                return Err(Error::<T>::NoContributionsToClaim.into());
            }

            let reward_per_unit = T::ContributionReward::get();
            let reward_amount = total_contributions.checked_mul(reward_per_unit)
                .ok_or(Error::<T>::RewardCalculationOverflow)?;

            if reward_amount == 0 {
                return Err(Error::<T>::NoContributionsToClaim.into());
            }

            // Transfer the reward from the module's account to the sender's account
            T::Currency::transfer(
                &Self::account_id(), // The module's account
                &sender,
                reward_amount.into(), // Convert u128 to BalanceOf<T::Currency>
                frame_support::traits::ExistenceRequirement::AllowDeath,
            )?;

            // Update total unclaimed rewards
            <TotalUnclaimedRewards>::mutate(|total| {
                *total = total.checked_sub(reward_amount).unwrap_or_default();
            });

            Self::deposit_event(RawEvent::RewardsClaimed(sender, reward_amount));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    /// The account ID of the Proof-of-Contribution pallet.
    /// This is used to hold and distribute rewards.
    pub fn account_id() -> T::AccountId {
        // This is a common pattern for generating a unique pallet account ID.
        // The `b"poc_mod"` is a unique byte string for this pallet.
        <pallet_balances::Module<T> as frame_support::traits::Module>::module_account_id()
    }

    /// Public function to add content contributions (callable by other pallets/runtime)
    pub fn add_content_contribution(who: &T::AccountId, amount: u128) -> DispatchResult {
        <ContentContributions<T>>::mutate(who.clone(), |contributions| {
            *contributions = contributions.checked_add(amount).ok_or(Error::<T>::RewardCalculationOverflow)?;
        });
        <TotalUnclaimedRewards>::mutate(|total| {
            *total = total.checked_add(amount.checked_mul(T::ContributionReward::get()).ok_or(Error::<T>::RewardCalculationOverflow)?).ok_or(Error::<T>::RewardCalculationOverflow)?;
        });
        Self::deposit_event(RawEvent::ContentContributed(who.clone(), amount));
        Ok(())
    }

    /// Public function to add network contributions (callable by other pallets/runtime)
    pub fn add_network_contribution(who: &T::AccountId, amount: u128) -> DispatchResult {
        <NetworkContributions<T>>::mutate(who.clone(), |contributions| {
            *contributions = contributions.checked_add(amount).ok_or(Error::<T>::RewardCalculationOverflow)?;
        });
        <TotalUnclaimedRewards>::mutate(|total| {
            *total = total.checked_add(amount.checked_mul(T::ContributionReward::get()).ok_or(Error::<T>::RewardCalculationOverflow)?).ok_or(Error::<T>::RewardCalculationOverflow)?;
        });
        Self::deposit_event(RawEvent::NetworkContributed(who.clone(), amount));
        Ok(())
    }

    /// Distribute rewards to all contributors based on their contributions
    fn distribute_rewards() -> DispatchResult {
        let reward_per_unit = T::ContributionReward::get();
        let total_rewards = <TotalUnclaimedRewards>::get();
        
        if total_rewards == 0 {
            return Ok(()); // Nothing to distribute
        }

        // Calculate total contributions across all users
        let mut total_contributions = 0u128;
        <ContentContributions<T>>::iter().for_each(|(_, amount)| {
            total_contributions = total_contributions.saturating_add(amount);
        });
        <NetworkContributions<T>>::iter().for_each(|(_, amount)| {
            total_contributions = total_contributions.saturating_add(amount);
        });

        if total_contributions == 0 {
            return Ok(()); // No contributions to reward
        }

        // Distribute rewards proportionally
        let mut distributed = 0u128;
        let mut errors = Vec::new();

        // Distribute content rewards
        <ContentContributions<T>>::iter().try_for_each(|(who, amount)| {
            let reward = amount.checked_mul(reward_per_unit)
                .and_then(|r| r.checked_mul(total_rewards))
                .and_then(|r| r.checked_div(total_contributions))
                .ok_or(Error::<T>::RewardCalculationOverflow)?;

            if reward > 0 {
                T::Currency::transfer(
                    &Self::account_id(),
                    &who,
                    reward.into(),
                    frame_support::traits::ExistenceRequirement::AllowDeath,
                )?;
                distributed = distributed.checked_add(reward)
                    .ok_or(Error::<T>::RewardCalculationOverflow)?;
                Self::deposit_event(RawEvent::RewardsClaimed(who, reward));
            }
            Ok(())
        }).map_err(|e| {
            errors.push(e);
            e
        })?;

        // Distribute network rewards
        <NetworkContributions<T>>::iter().try_for_each(|(who, amount)| {
            let reward = amount.checked_mul(reward_per_unit)
                .and_then(|r| r.checked_mul(total_rewards))
                .and_then(|r| r.checked_div(total_contributions))
                .ok_or(Error::<T>::RewardCalculationOverflow)?;

            if reward > 0 {
                T::Currency::transfer(
                    &Self::account_id(),
                    &who,
                    reward.into(),
                    frame_support::traits::ExistenceRequirement::AllowDeath,
                )?;
                distributed = distributed.checked_add(reward)
                    .ok_or(Error::<T>::RewardCalculationOverflow)?;
                Self::deposit_event(RawEvent::RewardsClaimed(who, reward));
            }
            Ok(())
        }).map_err(|e| {
            errors.push(e);
            e
        })?;

        // Update total unclaimed rewards
        <TotalUnclaimedRewards>::mutate(|total| {
            *total = total.saturating_sub(distributed);
        });

        // Clear all contributions after distribution
        <ContentContributions<T>>::remove_all();
        <NetworkContributions<T>>::remove_all();

        if !errors.is_empty() {
            log::error!("Errors during reward distribution: {:?}", errors);
            return Err(Error::<T>::RewardCalculationOverflow.into());
        }

        Ok(())
        }

        // Calculate total contributions across all users
        let mut total_contributions = 0u128;
        <ContentContributions<T>>::iter().for_each(|(_, amount)| {
            total_contributions = total_contributions.saturating_add(amount);
        });
        <NetworkContributions<T>>::iter().for_each(|(_, amount)| {
            total_contributions = total_contributions.saturating_add(amount);
        });

        if total_contributions == 0 {
            return Ok(()); // No contributions to reward
        }

        // Distribute rewards proportionally
        let mut distributed = 0u128;
        let mut errors = Vec::new();

        // Distribute content rewards
        <ContentContributions<T>>::iter().try_for_each(|(who, amount)| {
            let reward = amount.checked_mul(reward_per_unit)
                .and_then(|r| r.checked_mul(total_rewards))
                .and_then(|r| r.checked_div(total_contributions))
                .ok_or(Error::<T>::RewardCalculationOverflow)?;

            if reward > 0 {
                T::Currency::transfer(
                    &Self::account_id(),
                    &who,
                    reward.into(),
                    frame_support::traits::ExistenceRequirement::AllowDeath,
                )?;
                distributed = distributed.checked_add(reward)
                    .ok_or(Error::<T>::RewardCalculationOverflow)?;
                Self::deposit_event(RawEvent::RewardsClaimed(who, reward));
            }
            Ok(())
        }).map_err(|e| {
            errors.push(e);
            e
        })?;

        // Distribute network rewards
        <NetworkContributions<T>>::iter().try_for_each(|(who, amount)| {
            let reward = amount.checked_mul(reward_per_unit)
                .and_then(|r| r.checked_mul(total_rewards))
                .and_then(|r| r.checked_div(total_contributions))
                .ok_or(Error::<T>::RewardCalculationOverflow)?;

            if reward > 0 {
                T::Currency::transfer(
                    &Self::account_id(),
                    &who,
                    reward.into(),
                    frame_support::traits::ExistenceRequirement::AllowDeath,
                )?;
                distributed = distributed.checked_add(reward)
                    .ok_or(Error::<T>::RewardCalculationOverflow)?;
                Self::deposit_event(RawEvent::RewardsClaimed(who, reward));
            }
            Ok(())
        }).map_err(|e| {
            errors.push(e);
            e
        })?;

        // Update total unclaimed rewards
        <TotalUnclaimedRewards>::mutate(|total| {
            *total = total.saturating_sub(distributed);
        });

        // Clear all contributions after distribution
        <ContentContributions<T>>::remove_all();
        <NetworkContributions<T>>::remove_all();

        if !errors.is_empty() {
            log::error!("Errors during reward distribution: {:?}", errors);
            return Err(Error::<T>::RewardCalculationOverflow.into());
        }

        Ok(())
    }
}
