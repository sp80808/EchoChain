import Foundation
import Substrate

struct ContentRewards {
    struct RewardDistributedEvent: PalletEvent {
        static let name: String = "RewardDistributed"
        static let pallet: String = "ContentRewards"
        
        let user: AccountId
        let amount: UInt128
    }
}

struct NetworkRewards {
    struct NetworkRewardDistributedEvent: PalletEvent {
        static let name: String = "NetworkRewardDistributed"
        static let pallet: String = "NetworkRewards"
        
        let user: AccountId
        let amount: UInt128
    }
    
    struct ReportSubmittedEvent: PalletEvent {
        static let name: String = "ReportSubmitted"
        static let pallet: String = "NetworkRewards"
        
        let user: AccountId
        let uploaded: UInt64
        let downloaded: UInt64
    }
}

struct SampleRegistry {
    struct SampleRegisteredEvent: PalletEvent {
        static let name: String = "SampleRegistered"
        static let pallet: String = "SampleRegistry"
        
        let sample_id: UInt32
        let owner: AccountId
        let ipfs_cid: [UInt8]
    }
}
