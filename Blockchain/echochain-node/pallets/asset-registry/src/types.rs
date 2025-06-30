use frame_support::BoundedVec;
use sp_std::vec::Vec;

pub struct RegisteredAsset<AccountId, BlockNumber, Hash, MaxIpfsCidLen, MaxTagLen, MaxTags>
where
    MaxIpfsCidLen: frame_support::traits::Get<u32>,
    MaxTagLen: frame_support::traits::Get<u32>,
    MaxTags: frame_support::traits::Get<u32>,
{
    pub asset_id: Hash,
    pub owner: AccountId,
    pub registered_at: BlockNumber,
    pub ipfs_cid: BoundedVec<u8, MaxIpfsCidLen>,
    pub tags: BoundedVec<BoundedVec<u8, MaxTagLen>, MaxTags>,
}
