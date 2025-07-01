import Foundation
import Substrate
import SubstrateKeychain

extension RealBlockchainClient {
    // Submit network contribution (P2P seeding/bandwidth)
    func submitNetworkContribution(uploaded: UInt64, downloaded: UInt64) async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }

        // Assuming 'networkRewards' pallet has a 'submitReport' extrinsic
        let call = AnyCall(name: "submitReport",
                           pallet: "NetworkRewards", // Pallet name from your runtime
                           params: [
                            "uploaded": uploaded,
                            "downloaded": downloaded
                           ])

        let tx = try await api.tx.new(call)

        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()

        print("Network contribution submitted. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }

    // Submit content contribution (e.g., sample upload)
    func submitContentContribution(amount: UInt64) async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }

        // Assuming 'proofOfContribution' pallet has a 'recordContentContribution' extrinsic
        let call = AnyCall(name: "recordContentContribution",
                           pallet: "ProofOfContribution", // Pallet name from your runtime
                           params: ["amount": amount])

        let tx = try await api.tx.new(call)

        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()

        print("Content contribution submitted. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }

    // Claim rewards for user
    func claimRewards() async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }

        // Assuming 'proofOfContribution' pallet has a 'claimRewards' extrinsic
        let call = AnyCall(name: "claimRewards",
                           pallet: "ProofOfContribution", // Pallet name from your runtime
                           params: [])

        let tx = try await api.tx.new(call)

        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()

        print("Rewards claimed. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }
}