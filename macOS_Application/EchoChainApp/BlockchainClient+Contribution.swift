import Foundation

extension RealBlockchainClient {
    // Submit network contribution (P2P seeding/bandwidth)
    func submitNetworkContribution(uploaded: UInt64, downloaded: UInt64) async throws -> String {
        let params = [uploaded, downloaded]
        let request = JSONRPCRequest(method: "networkRewards_submit_report", params: params)
        let result: String = try await sendRPC(request: request)
        return result
    }

    // Submit content contribution (e.g., sample upload)
    func submitContentContribution(amount: UInt64) async throws -> String {
        let params = [amount]
        let request = JSONRPCRequest(method: "proofOfContribution_record_content_contribution", params: params)
        let result: String = try await sendRPC(request: request)
        return result
    }

    // Claim rewards for user
    func claimRewards() async throws -> String {
        let request = JSONRPCRequest(method: "proofOfContribution_claim_rewards", params: [])
        let result: String = try await sendRPC(request: request)
        return result
    }
} 