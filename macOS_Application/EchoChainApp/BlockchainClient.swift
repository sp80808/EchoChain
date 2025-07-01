import Foundation
import Combine
import Substrate
import SubstrateRPC
import SubstrateKeychain

// Protocol to define the interface for blockchain interactions
protocol BlockchainClientProtocol: ObservableObject {
    var balance: Double { get }
    var walletAddress: String { get }
    var transactionHistory: [Transaction] { get }
    var isConnected: Bool { get }

    func createWallet() async throws
    func importWallet(mnemonic: String) async throws
    func fetchBalance() async throws
    func fetchTransactionHistory() async throws
    func sendTransaction(to recipientAddress: String, amount: Double) async throws -> String
    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String
    func purchaseSample(sampleId: String, price: Double, recipientAddress: String) async throws -> String
    func checkNodeConnection() async throws -> (chain: String, version: String)
    func claimRewards() async throws -> String
    func submitNetworkContribution(uploaded: UInt64, downloaded: UInt64) async throws -> String
    func submitContentContribution(amount: UInt64) async throws -> String
}

// MARK: - Production Blockchain Client

class RealBlockchainClient: BlockchainClientProtocol {
    @Published var balance: Double = 0.0
    @Published var walletAddress: String = ""
    @Published var transactionHistory: [Transaction] = []
    @Published var isConnected: Bool = false

    private let nodeURL = URL(string: "ws://127.0.0.1:9945")! // Alice's WS port
    private let secureStorage = SecureStorage()
    
    func isBiometricsEnabled() -> Bool {
        return secureStorage.canEvaluatePolicy()
    }
    
    func authenticateWithBiometrics(reason: String) async throws -> Bool {
        let context = LAContext()
        var error: NSError?
        
        guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
            throw BlockchainClientError.unsupportedOperation("Biometrics not available")
        }
        
        return try await withCheckedThrowingContinuation { continuation in
            context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics,
                                 localizedReason: reason) { success, error in
                if let error = error {
                    continuation.resume(throwing: error)
                } else {
                    continuation.resume(returning: success)
                }
            }
        }
    }
    private var api: Api<DynamicConfig>?
    private var currentKeyPair: Sr25519KeyPair? // Manages the active keypair for signing transactions.

    init() {
        Task {
            do {
                // Initialize Substrate API
                self.api = try await Api(rpc: JsonRpcClient(.ws(url: nodeURL)), config: .dynamicBlake2)
                self.isConnected = true
                print("Substrate API initialized and connected.")

                // Attempt to load an existing wallet from SecureStorage on initialization.
                if let mnemonic = try secureStorage.getMnemonic() {
                    self.currentKeyPair = try Sr25519KeyPair(phrase: mnemonic)
                    self.walletAddress = self.currentKeyPair?.ss58Address() ?? ""
                    print("Wallet loaded from mnemonic: \(self.walletAddress)")
                    await self.fetchBalance()
                    await self.fetchTransactionHistory()
                } else {
                    print("No wallet mnemonic found in SecureStorage. User needs to create or import one.")
                }
            } catch {
                print("Failed to initialize Substrate API or load wallet: \(error.localizedDescription)")
                self.isConnected = false
                // TODO: Handle this error gracefully, perhaps by showing an alert to the user.
            }
        }
    }

    @MainActor
    func checkNodeConnection() async throws -> (chain: String, version: String) {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        
        let chainName: String = try await api.rpc.call(method: "system_chain", params: Params())
        let nodeVersion: String = try await api.rpc.call(method: "system_version", params: Params())
        
        return (chainName, nodeVersion)
    }

    @MainActor
    func createWallet() async throws {
        let newMnemonic = try secureStorage.generateMnemonicAndStore()
        self.currentKeyPair = try Sr25519KeyPair(phrase: newMnemonic)
        self.walletAddress = self.currentKeyPair?.ss58Address() ?? ""
        print("New wallet created: \(self.walletAddress)")
        await self.fetchBalance()
        await self.fetchTransactionHistory()
    }

    @MainActor
    func importWallet(mnemonic: String) async throws {
        try secureStorage.saveMnemonic(mnemonic)
        self.currentKeyPair = try Sr25519KeyPair(phrase: mnemonic)
        self.walletAddress = self.currentKeyPair?.ss58Address() ?? ""
        print("Wallet imported: \(self.walletAddress)")
        await self.fetchBalance()
        await self.fetchTransactionHistory()
    }

    @MainActor
    func fetchBalance() async throws {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }
        
        let accountId = try currentKeyPair.account(in: api)
        
        // Fetch account info using the dynamic storage query
        let accountInfo = try await api.query.dynamic(name: "Account", pallet: "System")
            .value(accountId, type: AccountInfo.self)
        
        // Assuming AccountInfo has a `data` field with `free` balance
        if let info = accountInfo {
            // Substrate balances are typically represented as UInt128.
            // Convert to Double for display, considering the chain's token decimals.
            // For EchoChain, let's assume 12 decimals for now (common for Substrate).
            let decimals: Double = 1_000_000_000_000 // 12 decimals
            self.balance = Double(info.data.free) / decimals
            print("Fetched balance for \(walletAddress): \(self.balance) ECHO")
        } else {
            self.balance = 0.0
            print("Account \(walletAddress) not found or has no balance.")
        }
    }

    @MainActor
    func fetchTransactionHistory() async throws {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }
        
        // Fetching transaction history directly from a Substrate node's RPC is complex.
        // It's highly recommended to use an off-chain indexer (e.g., SubQuery) for this.
        // For demonstration, we'll fetch recent blocks and try to find relevant events.
        
        // This is a simplified approach and might not be efficient for a full history.
        // A real implementation would involve subscribing to events or using an indexer.
        
        let latestBlock = try await api.rpc.chain.getBlock()
        guard let blockHash = latestBlock?.block.hash else {
            print("Could not get latest block hash.")
            self.transactionHistory = []
            return
        }
        
        let events = try await api.query.system.events(at: blockHash)
        
        var newTransactions: [Transaction] = []
        for eventRecord in events {
            // Example: Filter for Balances.Transfer events
            if let transferEvent = eventRecord.event.as(Balances.TransferEvent.self) {
                let fromAddress = try transferEvent.from.ss58Address(in: api)
                let toAddress = try transferEvent.to.ss58Address(in: api)
                let amount = Double(transferEvent.amount) / 1_000_000_000_000 // Assuming 12 decimals
                
                let type: TransactionType = (fromAddress == currentKeyPair.ss58Address()) ? .sent : .received
                
                newTransactions.append(Transaction(
                    id: UUID(),
                    type: type,
                    amount: amount,
                    from: fromAddress,
                    to: toAddress,
                    date: Date() // Placeholder date, ideally from block timestamp
                ))
            } else if let rewardEvent = eventRecord.event.as(ContentRewards.RewardDistributedEvent.self) {
                let userAddress = try rewardEvent.user.ss58Address(in: api)
                let amount = Double(rewardEvent.amount) / 1_000_000_000_000
                
                newTransactions.append(Transaction(
                    id: UUID(),
                    type: .contentReward,
                    amount: amount,
                    from: "Content Reward Pool",
                    to: userAddress,
                    date: Date()
                ))
            } else if let networkRewardEvent = eventRecord.event.as(NetworkRewards.NetworkRewardDistributedEvent.self) {
                let userAddress = try networkRewardEvent.user.ss58Address(in: api)
                let amount = Double(networkRewardEvent.amount) / 1_000_000_000_000
                
                newTransactions.append(Transaction(
                    id: UUID(),
                    type: .networkReward,
                    amount: amount,
                    from: "Network Reward Pool",
                    to: userAddress,
                    date: Date()
                ))
            } else if let reportEvent = eventRecord.event.as(NetworkRewards.ReportSubmittedEvent.self) {
                let userAddress = try reportEvent.user.ss58Address(in: api)
                
                newTransactions.append(Transaction(
                    id: UUID(),
                    type: .reportSubmitted,
                    amount: 0,
                    from: userAddress,
                    to: "Network Rewards Pallet",
                    date: Date()
                ))
            } else if let sampleEvent = eventRecord.event.as(SampleRegistry.SampleRegisteredEvent.self) {
                let ownerAddress = try sampleEvent.owner.ss58Address(in: api)
                
                newTransactions.append(Transaction(
                    id: UUID(),
                    type: .sampleRegistration,
                    amount: 0,
                    from: ownerAddress,
                    to: "Sample Registry Pallet",
                    date: Date()
                ))
            }
        }
        
        self.transactionHistory = newTransactions.sorted(by: { $0.date > $1.date })
        print("Fetched \(self.transactionHistory.count) transactions for \(walletAddress)")
    }

    @MainActor
    func sendTransaction(to recipientAddress: String, amount: Double) async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }
        
        let recipientAccountId = try api.runtime.address(ss58: recipientAddress)
        
        // Convert Double amount to UInt128 considering chain decimals
        let decimals: UInt128 = 1_000_000_000_000 // 12 decimals
        let rawAmount = UInt128(amount * Double(decimals))
        
        // Create the transfer call
        let call = AnyCall(name: "transfer",
                           pallet: "Balances",
                           params: ["dest": recipientAccountId, "value": rawAmount])
        
        // Create the extrinsic (transaction)
        let tx = try await api.tx.new(call)
        
        // Sign and send the extrinsic
        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()
        
        print("Transaction successful. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }

    @MainActor
    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }
        
        // Assuming 'sampleRegistry' pallet has a 'registerSample' extrinsic
        let call = AnyCall(name: "registerSample",
                           pallet: "SampleRegistry", // Pallet name from your runtime
                           params: [
                            "title": title,
                            "artist": artist,
                            "p2p_content_id": p2pContentId,
                            "blockchain_hash": blockchainHash
                           ])
        
        let tx = try await api.tx.new(call)
        
        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()
        
        print("Sample metadata registered. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }

    @MainActor
    func purchaseSample(sampleId: String, price: Double, recipientAddress: String) async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }

        let recipientAccountId = try api.runtime.address(ss58: recipientAddress)

        // Convert Double price to UInt128 considering chain decimals
        let decimals: UInt128 = 1_000_000_000_000 // 12 decimals
        let rawPrice = UInt128(price * Double(decimals))

        // Assuming 'SampleMarket' pallet has a 'buySample' extrinsic
        let call = AnyCall(name: "buySample",
                           pallet: "SampleMarket", // Pallet name from your runtime
                           params: [
                            "sample_id": sampleId,
                            "price": rawPrice,
                            "recipient": recipientAccountId
                           ])

        let tx = try await api.tx.new(call)

        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()

        print("Sample purchased. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }

    @MainActor
    func claimRewards() async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }

        // Assuming 'ContentRewards' pallet has a 'claimRewards' extrinsic
        let call = AnyCall(name: "claimRewards", pallet: "ContentRewards", params: [String: Any]())

        let tx = try await api.tx.new(call)

        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()

        print("Rewards claimed. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }

    @MainActor
    func submitNetworkContribution(uploaded: UInt64, downloaded: UInt64) async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }

        // Assuming 'NetworkRewards' pallet has a 'submitReport' extrinsic
        let call = AnyCall(name: "submitReport",
                           pallet: "NetworkRewards",
                           params: [
                            "bytes_uploaded": uploaded,
                            "bytes_downloaded": downloaded
                           ])

        let tx = try await api.tx.new(call)

        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()

        print("Network contribution submitted. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }

    @MainActor
    func submitContentContribution(amount: UInt64) async throws -> String {
        guard let api = self.api else { throw BlockchainClientError.nodeNotConnected }
        guard let currentKeyPair = self.currentKeyPair else { throw BlockchainClientError.walletNotLoaded }

        // Assuming 'ContentRewards' pallet has a 'submitContribution' extrinsic
        let call = AnyCall(name: "submitContribution",
                           pallet: "ContentRewards",
                           params: [
                            "amount": amount
                           ])

        let tx = try await api.tx.new(call)

        let events = try await tx.signSendAndWatch(signer: currentKeyPair)
            .waitForFinalized()
            .success()

        print("Content contribution submitted. Events: \(try events.parsed())")
        return events.extrinsicHash.toHex()
    }
}

// Helper struct for AccountInfo decoding
struct AccountInfo: Decodable {
    let nonce: UInt32
    let consumers: UInt32
    let providers: UInt32
    let sufficients: UInt32
    let data: AccountData
}

struct AccountData: Decodable {
    let free: UInt128
    let reserved: UInt128
    let miscFrozen: UInt128
    let feeFrozen: UInt128
}

enum BlockchainClientError: Error, LocalizedError {
    case nodeNotConnected
    case walletNotLoaded
    case transactionFailed(String)
    case metadataRegistrationFailed(String)
    case unsupportedOperation(String)

    var errorDescription: String? {
        switch self {
        case .nodeNotConnected:
            return "Not connected to a blockchain node."
        case .walletNotLoaded:
            return "Wallet not loaded. Please create or import a wallet first."
        case .transactionFailed(let message):
            return "Transaction failed: \(message)"
        case .metadataRegistrationFailed(let message):
            return "Sample metadata registration failed: \(message)"
        case .unsupportedOperation(let message):
            return message
        }
    }
}

struct Transaction: Identifiable {
    let id: UUID
    let type: TransactionType
    let amount: Double
    let from: String
    let to: String
    let date: Date
}

enum TransactionType: String {
    case sent
    case received
    case creation
    case `import`
    case sampleRegistration
    case contentReward
    case networkReward
    case reportSubmitted
}

