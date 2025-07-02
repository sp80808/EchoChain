"""#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use pallet_grandpa::AuthorityId as GrandpaId;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, PalletId};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, Verify,
	},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime, parameter_types,
	traits::{
		ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness,
		StorageInfo,
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		IdentityFee, Weight,
	},
	StorageValue,
};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier, OnChargeTransaction};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

/// Import the template pallet.
pub use pallet_template;
/// Import the sample registry pallet.
pub use pallet_sample_registry;
/// Import the content rewards pallet.
pub use pallet_content_rewards;
/// Import the network rewards pallet.
pub use pallet_network_rewards;
/// Import the p2p integration pallet.
pub use pallet_p2p_integration;
/// Import the Proof-of-Contribution pallet.
pub use pallet_proof_of_contribution;
/// Import the Echochain Compute pallet.
pub use pallet_echochain_compute;
/// Import the Echochain Marketplace pallet.
pub use pallet_echochain_marketplace;
/// Import the Democracy pallet.
pub use pallet_democracy;
/// Import the Collective pallet.
pub use pallet_collective;
/// Import the Treasury pallet.
pub use pallet_treasury;
pub use pallet_scheduler;

/// Import XCM and Contracts
use pallet_contracts;
use pallet_xcm;
use xcm_builder;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}
}

/// Runtime version information
///
/// This defines the version of the runtime specification (`spec_version`),
/// implementation (`impl_version`), and other metadata used for upgrades
/// and compatibility checking.
///
/// # Version Components
/// - `spec_name`: "node-template" (should match chain specification)
/// - `impl_name`: "node-template" (implementation identifier)
/// - `spec_version`: 100 (incremented for runtime specification changes)
/// - `impl_version`: 1 (incremented for implementation changes)
/// - `authoring_version`: 1 (version for block authors)
/// - `transaction_version`: 1 (version for transaction format)
/// - `state_version`: 1 (version for storage layout)
///
/// # Upgrade Rules
/// - Native runtime will only substitute Wasm if all version fields match
/// - Polkadot-JS Apps uses spec_version 100+ for custom type compatibility
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("echochain"),
	impl_name: create_runtime_str!("echochain"),
	authoring_version: 1,
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// Block time configuration
///
/// # Constants
/// - `MILLISECS_PER_BLOCK`: 6000 (6 second target block time)
/// - `SLOT_DURATION`: Equal to block time (6 seconds)
///
/// # Derived Time Constants
/// - `MINUTES`: 10 blocks (60_000ms / 6000ms per block)
/// - `HOURS`: 600 blocks (60 minutes)
/// - `DAYS`: 14,400 blocks (24 hours)
///
/// # Important Notes
/// - Slot duration cannot be changed after chain start
/// - Changing these values requires a runtime upgrade
/// - Actual block times may vary slightly from target
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time constants derived from block time
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::with_sensible_defaults(
			Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
			NORMAL_DISPATCH_RATIO,
		);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = frame_support::traits::Everything;
	/// The block type for the runtime.
	type Block = Block;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The type for storing how many extrinsics an account has signed.
	type Nonce = Nonce;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<32>;
	type AllowMultipleBlocksPerSlot = ConstBool<false>;

	#[cfg(feature = "experimental")]
	type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = ();
	type MaxAuthorities = ConstU32<32>;
	type MaxNominators = ConstU32<0>;
	type MaxSetIdSessionEntries = ConstU64<0>;

	type KeyOwnerProof = sp_core::Void;
	type EquivocationReportSystem = ();
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = ();
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 1;

impl pallet_balances::Config for Runtime {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}

/// A zero-fee implementation of OnChargeTransaction
pub struct ZeroFeeOnChargeTransaction;
impl OnChargeTransaction<Runtime> for ZeroFeeOnChargeTransaction {
    type Balance = Balance;
    type LiquidityInfo = ();

    fn withdraw_fee(
        &self,
        _who: &AccountId,
        _call: &RuntimeCall,
        _info: &DispatchInfoOf<RuntimeCall>,
        _fee: Self::Balance,
        _tip: Self::Balance,
    ) -> Result<Self::LiquidityInfo, TransactionValidityError> {
        Ok(())
    }

    fn correct_and_deposit_fee(
        &self,
        _who: &AccountId,
        _corrected_fee: Self::Balance,
        _tip: Self::Balance,
        _already_withdrawn: Self::LiquidityInfo,
    ) -> Result<(), TransactionValidityError> {
        Ok(())
    }
}

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = ZeroFeeOnChargeTransaction;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

impl pallet_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_template::weights::SubstrateWeight<Runtime>;
}

impl pallet_sample_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_content_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MinApprovedSamples = ConstU32<5>;
	type RewardAmount = ConstU128<100_000_000_000_000>; // 100 ECHO (assuming 12 decimals)
	type SampleRegistry = SampleRegistry;
}

impl pallet_network_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type TotalRewardPerPeriod = ConstU128<1000_000_000_000_000>; // 1000 ECHO (assuming 12 decimals)
}

impl pallet_p2p_integration::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

/// Configuration for Proof-of-Contribution pallet
///
/// # Reward Parameters
/// - `ContentRewardAmount`: 100 ECHO (100_000_000_000_000 with 12 decimals)
///   - Paid per approved content sample
/// - `MinSamplesForContentReward`: 5
///   - Minimum samples required to qualify for reward
/// - `NetworkRewardPeriod`: 1 day (14,400 blocks)
///   - Frequency of network contribution rewards
///
/// # Currency
/// Uses Balances pallet for fund management
///
/// # Example
/// ```rust
/// // Claim content reward
/// ProofOfContribution::claim_content_reward(origin, sample_count)?;
/// ```
impl pallet_proof_of_contribution::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ContentRewardAmount = ConstU128<100_000_000_000_000>; // 100 ECHO (assuming 12 decimals)
	type MinSamplesForContentReward = ConstU32<5>;
	type NetworkRewardPeriod = ConstU32<DAYS>; // Daily for testing, monthly in production
}

/// Configuration for the Echochain Compute pallet
///
/// # Parameters
/// - `RuntimeEvent`: Generic runtime event type
/// - `ManagerId`: u128 identifier for compute managers
/// - `ManagerIdProvider`: Proof-of-Contribution pallet provides manager IDs
/// - `Epoch`: 5 minutes (50 blocks) - reward distribution period
/// - `EpochBase`: 0 - starting epoch number
/// - `WarmupPeriod`: 2 minutes (20 blocks) - time before first rewards
/// - `Balance`: u128 - native balance type
/// - `BlockNumber`: u32 - native block number type
/// - `Currency`: Balances pallet handles funds
/// - `ComputeRewardDistributor`: Proof-of-Contribution pallet handles rewards
/// - `WeightInfo`: Benchmark-generated weights
///
/// # Usage
/// ```rust
/// // Get current epoch
/// let current_epoch = EchochainCompute::current_epoch();
/// ```
impl pallet_echochain_compute::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ManagerId = u128;
	type ManagerIdProvider = pallet_proof_of_contribution::PocManagerIdProvider;
	type Epoch = ConstU32<{ 5 * MINUTES }>; // 5 minutes per epoch for testing
	type EpochBase = ConstU32<0>;
	type WarmupPeriod = ConstU32<{ 2 * MINUTES }>; // 2 minutes warmup period
	type Balance = Balance;
	type BlockNumber = BlockNumber;
	type Currency = Balances;
	type ComputeRewardDistributor = pallet_proof_of_contribution::PocComputeRewardDistributor;
	type WeightInfo = pallet_echochain_compute::weights::SubstrateWeight<Runtime>;
}

impl pallet_echochain_marketplace::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = pallet_echochain_marketplace::weights::SubstrateWeight<Runtime>;
}

impl pallet_royalty_distribution::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::DefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = frame_system::EnsureRoot<AccountId>;
}

parameter_types! {
	pub const ProposalBond: Balance = 1;
	pub const ProposalBondMinimum: Balance = 1;
	pub const ProposalBondMaximum: Option<Balance> = None;
	pub const SpendPeriod: BlockNumber = 7 * DAYS;
	pub const Burn: Permill = Permill::from_percent(1); // Reduced burn rate to prevent deflation
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaxApprovals: u32 = 100;
}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = frame_system::EnsureRoot<AccountId>;
	type RejectOrigin = frame_system::EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = ();
	type MaxApprovals = MaxApprovals;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
}

parameter_types! {
	pub const LaunchPeriod: BlockNumber = 7 * DAYS;
	pub const VotingPeriod: BlockNumber = 7 * DAYS;
	pub const FastTrackVotingPeriod: BlockNumber = 3 * HOURS;
	pub const MinimumDeposit: Balance = 100;
	pub const EnactmentPeriod: BlockNumber = 8 * DAYS;
	pub const CooloffPeriod: BlockNumber = 7 * DAYS;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod; // Same as EnactmentPeriod
	type MinimumDeposit = MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = frame_collective::EnsureMembers<AccountId, CouncilCollective, 1>;
	/// A majority can have the next item be tabled.
	type BlacklistOrigin = frame_system::EnsureRoot<AccountId>;
	/// Any single council member can have the next scheduled referendum be a straight majority-carries vote
	type FastTrackOrigin = frame_collective::EnsureMembers<AccountId, CouncilCollective, 1>;
	type InstantOrigin = frame_collective::EnsureMembers<AccountId, CouncilCollective, 1>;
	type InstantAllowed = ConstBool<true>;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = frame_collective::EnsureMembers<AccountId, CouncilCollective, 2>;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancelProposalOrigin = frame_collective::EnsureMembers<AccountId, CouncilCollective, 2>;
	type VetoOrigin = frame_system::EnsureRoot<AccountId>;
	type CooloffPeriod = CooloffPeriod;
	type PreimageByteDeposit = ConstU128<1>;
	type OperationalPreimageOrigin = frame_collective::EnsureMembers<AccountId, CouncilCollective, 1>;
	type Slash = Treasury;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type MaxVotes = MaxVotes;
	type WeightInfo = ();
	type MaxProposals = MaxProposals;
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = BlockWeights;
	type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = ConstU32<50>;
	type WeightInfo = ();
	type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
	type Preimages = Preimage;
}

// --- XCM Minimal Config ---
parameter_types! {
    pub const XcmMaxAssets: u32 = 2;
}
impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SendXcmOrigin = frame_support::traits::NeverEnsureOrigin<u128>;
    type XcmExecuteFilter = frame_support::traits::Everything;
    type XcmExecutor = xcm_builder::XcmExecutor<Runtime>;
    type XcmTeleportFilter = frame_support::traits::Nothing;
    type XcmReserveTransferFilter = frame_support::traits::Nothing;
    type Weigher = xcm_builder::FixedWeightBounds<ConstU64<1>, Call, ConstU64<1>>;
    type LocationInverter = xcm_builder::LocationInverter<ConstU32<0>>;
    type Origin = RuntimeOrigin;
    type Call = RuntimeCall;
    type AssetTransactor = (); // No asset transfer for now
    type IsReserve = frame_support::traits::NeverEnsureOrigin<u128>;
    type IsTeleporter = frame_support::traits::NeverEnsureOrigin<u128>;
    type UniversalLocation = xcm::v1::MultiLocation;
    type Barrier = xcm_builder::AllowTopLevelPaidExecutionFrom<frame_support::traits::Everything>;
    type MaxAssetsForTransfer = XcmMaxAssets;
    type WeightInfo = ();
    type AdvertisedXcmVersion = xcm::Version::V3;
}

// --- Contracts Minimal Config ---
parameter_types! {
    pub const DepositPerByte: u64 = 1_000;
    pub const DepositPerItem: u64 = 10_000;
    pub const RentFraction: Perbill = Perbill::from_percent(1);
    pub const SurchargeReward: u64 = 150_000;
    pub const MaxCodeSize: u32 = 2 * 1024 * 1024; // 2 MB
    pub const MaxStorageKeyLen: u32 = 128;
    pub const MaxDebugBufferLen: u32 = 2 * 1024 * 1024; // 2 MB
}
impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = pallet_balances::Randomness;
    type Currency = Balances;
    type Event = RuntimeEvent;
    type Call = RuntimeCall;
    type CallFilter = frame_support::traits::Everything;
    type DepositPerByte = DepositPerByte;
    type DepositPerItem = DepositPerItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type MaxCodeSize = MaxCodeSize;
    type MaxStorageKeyLen = MaxStorageKeyLen;
    type MaxDebugBufferLen = MaxDebugBufferLen;
    type WeightPrice = pallet_transaction_payment::Module<Runtime>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Runtime>;
    type ChainExtension = (); // No chain extension for now
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		// Include the custom logic from the pallet-template in the runtime.
		TemplateModule: pallet_template,
		SampleRegistry: pallet_sample_registry,
		ContentRewards: pallet_content_rewards,
		NetworkRewards: pallet_network_rewards,
		P2PIntegration: pallet_p2p_integration,
		ProofOfContribution: pallet_proof_of_contribution,
		Democracy: pallet_democracy,
		Council: pallet_collective::<Instance1>,
		Treasury: pallet_treasury,
		Scheduler: pallet_scheduler,
		EchochainCompute: pallet_echochain_compute,
		EchochainMarketplace: pallet_echochain_marketplace,
		RoyaltyDistribution: pallet_royalty_distribution,
		Contracts: pallet_contracts,
		XcmPallet: pallet_xcm,
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_timestamp, Timestamp]
		[pallet_sudo, Sudo]
		[pallet_template, TemplateModule]
		[pallet_proof_of_contribution, ProofOfContribution]
		[pallet_content_rewards, ContentRewards]
	);
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> sp_consensus_grandpa::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: sp_consensus_grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: sp_consensus_grandpa::SetId,
			_authority_id: GrandpaId,
		) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;
			use pallet_proof_of_contribution::WeightInfo; // Import WeightInfo

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
			use sp_storage::TrackedStorageKey;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let mut whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();
			// Add ProofOfContribution pallet's storage keys to whitelist
			whitelist.extend(pallet_proof_of_contribution::WeightInfo::whitelisted_storage_keys());

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, BlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
		}
	}
}
""
