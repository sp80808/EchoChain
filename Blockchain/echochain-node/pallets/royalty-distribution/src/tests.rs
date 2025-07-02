#![cfg(test)]

use super::*;
use crate as pallet_royalty_distribution;
use frame_support::{
    assert_ok, parameter_types,
    traits::{ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        RoyaltyDistribution: pallet_royalty_distribution::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type HoldIdentifier = ();
    type MaxHolds = ();
}

impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 1000), (2, 1000), (3, 1000), (4, 1000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}

#[test]
fn test_initialize_royalties() {
    new_test_ext().execute_with(|| {
        assert_ok!(RoyaltyDistribution::initialize_royalties(
            RuntimeOrigin::root(),
            70,
            10,
            20
        ));

        let percentages = RoyaltyDistribution::get_royalty_percentages();
        assert_eq!(percentages, (70, 10, 20));
    });
}

#[test]
fn test_distribute_royalties() {
    new_test_ext().execute_with(|| {
        // Initialize royalties
        assert_ok!(RoyaltyDistribution::initialize_royalties(
            RuntimeOrigin::root(),
            70,
            10,
            20
        ));

        // Distribute 100 units
        assert_ok!(RoyaltyDistribution::distribute_royalties(
            RuntimeOrigin::signed(1),
            100,
            2,
            3,
            4
        ));

        // Check balances
        assert_eq!(Balances::free_balance(1), 900); // 1000 - 100
        assert_eq!(Balances::free_balance(2), 1070); // 1000 + 70
        assert_eq!(Balances::free_balance(3), 1010); // 1000 + 10
        assert_eq!(Balances::free_balance(4), 1020); // 1000 + 20

        // Check event
        let event = System::events()
            .pop()
            .expect("Expected at least one EventRecord to be found")
            .event;
        assert_eq!(
            event,
            RuntimeEvent::RoyaltyDistribution(
                Event::RoyaltiesDistributed(2, 3, 4)
            )
        );
    });
}

#[test]
fn test_register_nft() {
    new_test_ext().execute_with(|| {
        let nft_id = H256::random();
        let metadata = vec![1, 2, 3];

        assert_ok!(RoyaltyDistribution::register_nft(
            RuntimeOrigin::signed(1),
            nft_id,
            metadata.clone()
        ));

        assert_eq!(
            RoyaltyDistribution::nft_metadata(nft_id),
            metadata
        );

        // Check event
        let event = System::events()
            .pop()
            .expect("Expected at least one EventRecord to be found")
            .event;
        assert_eq!(
            event,
            RuntimeEvent::RoyaltyDistribution(
                Event::NftRegistered(nft_id, 1)
            )
        );
    });
}