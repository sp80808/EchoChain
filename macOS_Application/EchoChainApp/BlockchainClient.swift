import Foundation
import Combine

// Protocol to define the interface for blockchain interactions
protocol BlockchainClientProtocol: ObservableObject {
    var balance: Double { get }
    var walletAddress: String { get }
    var transactionHistory: [Transaction] { get }

    func createWallet() async throws
    func importWallet(privateKey: String) async throws
    func fetchBalance() async throws
    func fetchTransactionHistory() async throws
    func signTransaction(from: String, to: String, amount: Double, data: String?) async throws -> String
    func broadcastTransaction(signedTransaction: String) async throws -> String
    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String
}

// Simulated Blockchain Client
class SimulatedBlockchainClient: BlockchainClientProtocol {
    @Published var balance: Double = 0.0
    @Published var walletAddress: String = "echoxxx...xxx"
    @Published var transactionHistory: [Transaction] = []

    private let secureStorage = SecureStorage()
    private var privateKey: String?

    init() {
        // Attempt to load existing wallet on initialization
        if let storedPrivateKey = secureStorage.retrievePrivateKey() {
            self.privateKey = storedPrivateKey
            self.walletAddress = "echo\(storedPrivateKey.prefix(10))...\(storedPrivateKey.suffix(10))"
            Task {
                await self.fetchBalance()
                await self.fetchTransactionHistory()
            }
        } else {
            print("No existing wallet found. Please create or import one.")
        }
    }

    @MainActor
    func createWallet() async throws {
        // In a real scenario, this would generate a new key pair and store the private key securely.
        let newPrivateKey = UUID().uuidString // Placeholder for actual key generation
        let newWalletAddress = "echo\(newPrivateKey.prefix(10))...\(newPrivateKey.suffix(10))"

        if secureStorage.storePrivateKey(newPrivateKey) {
            self.privateKey = newPrivateKey
            self.walletAddress = newWalletAddress
            self.balance = Double.random(in: 50.0...500.0)
            self.transactionHistory.append(Transaction(id: UUID(), type: .creation, amount: self.balance, from: "System", to: newWalletAddress, date: Date()))
            print("Simulated: New wallet created: \(walletAddress)")
        } else {
            throw BlockchainClientError.keyStorageFailed
        }
    }

    @MainActor
    func importWallet(privateKey: String) async throws {
        // In a real scenario, this would validate the private key and derive the address.
        let importedWalletAddress = "echo\(privateKey.prefix(10))...\(privateKey.suffix(10))"

        if secureStorage.storePrivateKey(privateKey) {
            self.privateKey = privateKey
            self.walletAddress = importedWalletAddress
            self.balance = Double.random(in: 10.0...1000.0)
            self.transactionHistory.append(Transaction(id: UUID(), type: .import, amount: self.balance, from: "System", to: importedWalletAddress, date: Date()))
            print("Simulated: Existing wallet imported: \(walletAddress)")
        } else {
            throw BlockchainClientError.keyStorageFailed
        }
    }

    @MainActor
    func fetchBalance() async throws {
        // Simulate fetching balance from blockchain
        try await Task.sleep(nanoseconds: 500_000_000) // Simulate network delay
        self.balance = Double.random(in: 0.0...1000.0)
        print("Simulated: Fetched new balance: \(balance) ECHO")
    }

    @MainActor
    func fetchTransactionHistory() async throws {
        // Simulate fetching transaction history
        try await Task.sleep(nanoseconds: 700_000_000) // Simulate network delay
        self.transactionHistory = [
            Transaction(id: UUID(), type: .sent, amount: 10.0, from: "echoxxx...xxx", to: "echoyyy...yyy", date: Date().addingTimeInterval(-3600)),
            Transaction(id: UUID(), type: .received, amount: 5.0, from: "echozzz...zzz", to: "echoxxx...xxx", date: Date().addingTimeInterval(-7200)),
            Transaction(id: UUID(), type: .sent, amount: 2.0, from: "echoxxx...xxx", to: "echoaaa...aaa", date: Date().addingTimeInterval(-10800))
        ]
        print("Simulated: Fetched transaction history.")
    }

    @MainActor
    func signTransaction(from: String, to: String, amount: Double, data: String?) async throws -> String {
        guard privateKey != nil else {
            throw BlockchainClientError.walletNotLoaded
        }
        // Simulate transaction signing
        try await Task.sleep(nanoseconds: 300_000_000)
        let signedTx = "signed_tx_\(UUID().uuidString)"
        print("Simulated: Transaction signed: \(signedTx)")
        return signedTx
    }

    @MainActor
    func broadcastTransaction(signedTransaction: String) async throws -> String {
        // Simulate broadcasting transaction to the blockchain
        try await Task.sleep(nanoseconds: 1_000_000_000)
        let txHash = "tx_hash_\(UUID().uuidString)"
        print("Simulated: Transaction broadcasted: \(txHash)")
        return txHash
    }

    @MainActor
    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String {
        guard privateKey != nil else {
            throw BlockchainClientError.walletNotLoaded
        }
        // Simulate smart contract interaction for sample registration
        try await Task.sleep(nanoseconds: 1_500_000_000)
        let registrationTxHash = "sample_reg_tx_\(UUID().uuidString)"
        print("Simulated: Sample metadata registered on blockchain. Tx Hash: \(registrationTxHash)")
        return registrationTxHash
    }
}

enum BlockchainClientError: Error, LocalizedError {
    case keyStorageFailed
    case walletNotLoaded
    case transactionFailed(String)
    case metadataRegistrationFailed(String)

    var errorDescription: String? {
        switch self {
        case .keyStorageFailed:
            return "Failed to securely store wallet key."
        case .walletNotLoaded:
            return "Wallet not loaded. Please create or import a wallet first."
        case .transactionFailed(let message):
            return "Transaction failed: \(message)"
        case .metadataRegistrationFailed(let message):
            return "Sample metadata registration failed: \(message)"
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
}