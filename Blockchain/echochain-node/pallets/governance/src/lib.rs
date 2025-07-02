#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch::DispatchResult,
    traits::{Currency, Get},
};
use frame_system::ensure_root;
use sp_std::prelude::*;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    
    /// The PoC pallet instance
    type Poc: proof_of_contribution::Config;
    
    /// The currency type for governance deposits
    type Currency: Currency<<Self as frame_system::Config>::AccountId>;
    
    /// Minimum deposit required to propose parameter changes
    type ProposalDeposit: Get<<Self::Currency as Currency<<Self as frame_system::Config>::AccountId>>::Balance>;
}

decl_storage! {
    trait Store for Module<T: Config> as Governance {
        /// Active parameter change proposals
        pub Proposals get(fn proposals):
            map hasher(blake2_128_concat) u32 => Proposal<T>;
            
        /// Next proposal ID
        pub NextProposalId get(fn next_proposal_id): u32;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        /// New parameter change proposed [proposal_id, proposer]
        ProposalCreated(u32, AccountId),
        /// Parameter change enacted [proposal_id]
        ProposalEnacted(u32),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        /// Invalid reward parameters
        InvalidParameters,
        /// Proposal not found
        UnknownProposal,
        /// Insufficient deposit
        InsufficientDeposit,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Propose new reward parameters
        #[weight = 10_000]
        pub fn propose_reward_params(
            origin,
            content_reward: u128,
            storage_reward: u128,
            bandwidth_reward: u128,
            reward_interval: u32
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            
            // Take deposit
            T::Currency::reserve(&proposer, T::ProposalDeposit::get())?;
            
            // Create and store proposal
            let proposal_id = <NextProposalId>::get();
            let proposal = Proposal {
                proposer: proposer.clone(),
                content_reward,
                storage_reward,
                bandwidth_reward,
                reward_interval,
                deposit: T::ProposalDeposit::get(),
            };
            
            <Proposals<T>>::insert(proposal_id, proposal);
            <NextProposalId>::put(proposal_id + 1);
            
            Self::deposit_event(RawEvent::ProposalCreated(proposal_id, proposer));
            Ok(())
        }

        /// Enact approved reward parameters
        #[weight = 10_000]
        pub fn enact_proposal(
            origin,
            proposal_id: u32
        ) -> DispatchResult {
            ensure_root(origin)?;
            
            let proposal = <Proposals<T>>::take(proposal_id)
                .ok_or(Error::<T>::UnknownProposal)?;
                
            // Validate parameters
            if proposal.content_reward == 0 || proposal.storage_reward == 0 || proposal.bandwidth_reward == 0 {
                return Err(Error::<T>::InvalidParameters.into());
            }
            
            // Update PoC pallet configuration
            proof_of_contribution::Module::<T::Poc>::set_reward_parameters(
                proposal.content_reward,
                proposal.storage_reward,
                proposal.bandwidth_reward,
                proposal.reward_interval
            )?;
            
            // Return deposit
            T::Currency::unreserve(&proposal.proposer, proposal.deposit);
            
            Self::deposit_event(RawEvent::ProposalEnacted(proposal_id));
            Ok(())
        }
    }
}

/// A parameter change proposal
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug)]
pub struct Proposal<T: Config> {
    /// The account proposing the change
    pub proposer: T::AccountId,
    /// New content reward rate
    pub content_reward: u128,
    /// New storage reward rate
    pub storage_reward: u128,
    /// New bandwidth reward rate
    pub bandwidth_reward: u128,
    /// New reward distribution interval (blocks)
    pub reward_interval: u32,
    /// Deposit held for this proposal
    pub deposit: <T::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance,
}
