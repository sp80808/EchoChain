// Basic tests for asset-registry pallet
use super::*;
use crate as asset_registry;
use frame_support::{assert_ok, assert_noop, parameter_types, traits::{OnFinalize, OnInitialize}};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::{BlakeTwo256, IdentityLookup}};

// Mock runtime
#[frame_support::construct_runtime]
pub enum TestRuntime {
    System: frame_system,
    AssetRegistry: asset_registry,
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaxIpfsCidLen: u32 = 64;
    pub const MaxTagLen: u32 = 16;
    pub const MaxTags: u32 = 5;
    pub const MaxAssetsPerOwner: u32 = 10;
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<u64>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

impl asset_registry::Config for TestRuntime {
    type RuntimeEvent = ();
    type MaxIpfsCidLen = MaxIpfsCidLen;
    type MaxTagLen = MaxTagLen;
    type MaxTags = MaxTags;
    type MaxAssetsPerOwner = MaxAssetsPerOwner;
}

type AssetRegistry = asset_registry::Pallet<TestRuntime>;
type System = frame_system::Pallet<TestRuntime>;

fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
    t.into()
}

#[test]
fn register_and_query_asset_works() {
    new_test_ext().execute_with(|| {
        let owner = 1u64;
        let ipfs_cid = vec![1,2,3];
        let tags = vec![vec![4,5]];
        let bounded_cid = BoundedVec::try_from(ipfs_cid.clone()).unwrap();
        let bounded_tags = BoundedVec::try_from(tags.clone().into_iter().map(BoundedVec::try_from).map(Result::unwrap).collect::<Vec<_>>()).unwrap();
        assert_ok!(AssetRegistry::register_asset(Origin::signed(owner), bounded_cid.clone(), bounded_tags.clone()));
        let asset_ids = AssetRegistry::get_assets_by_owner(&owner);
        assert_eq!(asset_ids.len(), 1);
        let asset = AssetRegistry::get_asset(&asset_ids[0]).unwrap();
        assert_eq!(asset.owner, owner);
        assert_eq!(asset.ipfs_cid, bounded_cid);
        assert_eq!(asset.tags, bounded_tags);
    });
}

#[test]
fn deregister_asset_works() {
    new_test_ext().execute_with(|| {
        let owner = 1u64;
        let ipfs_cid = vec![1,2,3];
        let tags = vec![vec![4,5]];
        let bounded_cid = BoundedVec::try_from(ipfs_cid.clone()).unwrap();
        let bounded_tags = BoundedVec::try_from(tags.clone().into_iter().map(BoundedVec::try_from).map(Result::unwrap).collect::<Vec<_>>()).unwrap();
        assert_ok!(AssetRegistry::register_asset(Origin::signed(owner), bounded_cid.clone(), bounded_tags.clone()));
        let asset_ids = AssetRegistry::get_assets_by_owner(&owner);
        assert_eq!(asset_ids.len(), 1);
        assert_ok!(AssetRegistry::deregister_asset(Origin::signed(owner), asset_ids[0]));
        let asset_ids = AssetRegistry::get_assets_by_owner(&owner);
        assert_eq!(asset_ids.len(), 0);
    });
}

#[test]
fn update_asset_works() {
    new_test_ext().execute_with(|| {
        let owner = 1u64;
        let ipfs_cid = vec![1,2,3];
        let tags = vec![vec![4,5]];
        let bounded_cid = BoundedVec::try_from(ipfs_cid.clone()).unwrap();
        let bounded_tags = BoundedVec::try_from(tags.clone().into_iter().map(BoundedVec::try_from).map(Result::unwrap).collect::<Vec<_>>()).unwrap();
        assert_ok!(AssetRegistry::register_asset(Origin::signed(owner), bounded_cid.clone(), bounded_tags.clone()));
        let asset_ids = AssetRegistry::get_assets_by_owner(&owner);
        let new_cid = BoundedVec::try_from(vec![9,9,9]).unwrap();
        assert_ok!(AssetRegistry::update_asset(Origin::signed(owner), asset_ids[0], Some(new_cid.clone()), None));
        let asset = AssetRegistry::get_asset(&asset_ids[0]).unwrap();
        assert_eq!(asset.ipfs_cid, new_cid);
    });
}
