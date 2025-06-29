import Foundation
import Combine
import Security
import CryptoKit

// Protocol to define the interface for blockchain interactions
protocol BlockchainClientProtocol: ObservableObject {
    var balance: Double { get }
    var walletAddress: String { get }
    var transactionHistory: [Transaction] { get }

    func createWallet() async throws
    func fetchBalance() async throws
    func fetchTransactionHistory() async throws
    func signTransaction(from: String, to: String, amount: Double, data: String?) async throws -> String
    func broadcastTransaction(signedTransaction: String) async throws -> String
    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String
    func checkNodeConnection() async throws -> (chain: String, version: String)
}

// MARK: - Production Blockchain Client

class RealBlockchainClient: BlockchainClientProtocol {
    @Published var balance: Double = 0.0
    @Published var walletAddress: String = ""
    @Published var transactionHistory: [Transaction] = []

    private let nodeURL = URL(string: "http://localhost:9933")! // Substrate RPC HTTP port
    private let secureStorage = SecureStorage()

    private var privateKey: Curve25519.Signing.PrivateKey? // TODO: This should be managed more robustly, potentially not stored directly. The SecureStorage should ideally return this type directly.

    init() {
        // TODO: Implement proper initialization, potentially loading an existing wallet or prompting for creation.
        // Attempt to load private key from Secure Enclave
        if let publicKey = secureStorage.getPublicKey() {
            self.walletAddress = Self.deriveAddress(from: publicKey)
            Task {
                await self.fetchBalance() // TODO: Implement actual balance fetching from blockchain
                await self.fetchTransactionHistory() // TODO: Implement actual transaction history fetching
            }
        } else {
            // No wallet found, prompt user to create or import
            // This could be a notification, delegate callback, or UI event
            print("No wallet found. Please create or import a wallet.")
            // Example: NotificationCenter.default.post(name: .walletNotFound, object: nil)
        }
        Task {
            do {
                let (chain, version) = try await checkNodeConnection()
                print("Connected to EchoChain node: Chain=\(chain), Version=\(version)")
            } catch {
                print("Failed to connect to EchoChain node: \(error.localizedDescription)")
                // TODO: Handle node connection failure gracefully (e.g., show error to user).
            }
        }
    }

    @MainActor
    func checkNodeConnection() async throws -> (chain: String, version: String) {
        // TODO: This is a basic check. Consider using a dedicated Substrate API client for more robust checks.
        let chainRequest = JSONRPCRequest(method: "system_chain", params: [])
        let chainName: String = try await sendRPC(request: chainRequest)

        let versionRequest = JSONRPCRequest(method: "system_version", params: [])
        let nodeVersion: String = try await sendRPC(request: versionRequest)

        return (chainName, nodeVersion)
    }

    @MainActor
    func createWallet() async throws {
        // Generate a Curve25519 keypair and store private key in Keychain
        let privateKey = Curve25519.Signing.PrivateKey()
        let publicKey = privateKey.publicKey
        try secureStorage.savePrivateKey(privateKey)
        self.walletAddress = Self.deriveAddress(from: publicKey)
        self.privateKey = privateKey as? SecKey
    }

    func importWallet(privateKeyData: Data) async throws {
        // Import a Curve25519 private key from data and store in Keychain
        let privateKey = try Curve25519.Signing.PrivateKey(rawRepresentation: privateKeyData)
        try secureStorage.savePrivateKey(privateKey)
        self.walletAddress = Self.deriveAddress(from: privateKey.publicKey)
        self.privateKey = privateKey as? SecKey
    }

    @MainActor
    func fetchBalance() async throws {
        guard !walletAddress.isEmpty else { throw BlockchainClientError.walletNotLoaded }
        // TODO: Implement actual balance fetching from the blockchain.
        // This typically involves querying the `system.account` storage map.
        // Example RPC method: `state_getStorage` with the account key.
        // The current `chain_getBalance` is a placeholder and might not exist on a real node.
        let params = [walletAddress]
        let request = JSONRPCRequest(method: "chain_getBalance", params: params) // Placeholder RPC method
        let result: Double = try await sendRPC(request: request)
        self.balance = result
    }

    @MainActor
    func fetchTransactionHistory() async throws {
        guard !walletAddress.isEmpty else { throw BlockchainClientError.walletNotLoaded }
        let params = [walletAddress]
        let request = JSONRPCRequest(method: "chain_getTransactions", params: params)
        let result: [[String: Any]] = try await sendRPC(request: request)
        self.transactionHistory = result.compactMap { dict in
            guard let idStr = dict["id"] as? String,
                  let id = UUID(uuidString: idStr),
                  let typeStr = dict["type"] as? String,
                  let type = TransactionType(rawValue: typeStr),
                  let amount = dict["amount"] as? Double,
                  let from = dict["from"] as? String,
                  let to = dict["to"] as? String,
                  let dateInt = dict["date"] as? TimeInterval else { return nil }
            return Transaction(id: id, type: type, amount: amount, from: from, to: to, date: Date(timeIntervalSince1970: dateInt))
        }
    }

    @MainActor
    func signTransaction(from: String, to: String, amount: Double, data: String?) async throws -> String {
        guard let privateKey = self.privateKey as? Curve25519.Signing.PrivateKey else {
            throw BlockchainClientError.walletNotLoaded
        }
        let transactionDetails = "\(from)\(to)\(amount)\(data ?? "")"
        guard let dataToSign = transactionDetails.data(using: .utf8) else {
            throw BlockchainClientError.transactionFailed("Failed to encode transaction data for signing.")
        }
        let signature = try privateKey.signature(for: dataToSign)
        return signature.base64EncodedString()
    }

    @MainActor
    func broadcastTransaction(signedTransaction: String) async throws -> String {
        let params = [signedTransaction]
        let request = JSONRPCRequest(method: "author_submitExtrinsic", params: params)
        let result: String = try await sendRPC(request: request)
        return result
    }

    @MainActor
    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String {
        let params = [title, artist, p2pContentId, blockchainHash]
        let request = JSONRPCRequest(method: "custom_registerSample", params: params)
        let result: String = try await sendRPC(request: request)
        return result
    }

    // MARK: - JSON-RPC Networking
    struct JSONRPCRequest: Encodable {
        let jsonrpc = "2.0"
        let method: String
        let params: [AnyEncodable]
        let id = 1
        init(method: String, params: [Any]) {
            self.method = method
            self.params = params.map { AnyEncodable($0) }
        }
    }

    struct AnyEncodable: Encodable {
        private let _encode: (Encoder) throws -> Void
        init<T: Encodable>(_ wrapped: T) {
            _encode = wrapped.encode
        }
        func encode(to encoder: Encoder) throws { try _encode(encoder) }
    }

    private func sendRPC<T: Decodable>(request: JSONRPCRequest) async throws -> T {
        // TODO: This RPC client is a placeholder. Replace with a robust blockchain SDK's RPC client.
        var urlRequest = URLRequest(url: nodeURL)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")
        urlRequest.httpBody = try JSONEncoder().encode(request)
        let (data, response) = try await URLSession.shared.data(for: urlRequest)
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw BlockchainClientError.transactionFailed("Invalid response from node")
        }
        let rpcResponse = try JSONDecoder().decode(JSONRPCResponse<T>.self, from: data)
        if let error = rpcResponse.error {
            throw BlockchainClientError.transactionFailed(error.message)
        }
        guard let result = rpcResponse.result else {
            throw BlockchainClientError.transactionFailed("No result in response")
        }
        return result
    }

    struct JSONRPCResponse<T: Decodable>: Decodable {
        let result: T?
        let error: RPCError?
        struct RPCError: Decodable {
            let code: Int
            let message: String
        }
    }

    // MARK: - Address Derivation
    static func deriveAddress(from publicKey: SecKey) -> String {
        // TODO: Implement real address derivation from SecKey (public key)
        // This is a placeholder. A real implementation would involve hashing the public key
        // and encoding it according to the blockchain's address format.
        let keyData = SecKeyCopyExternalRepresentation(publicKey, nil)! as Data
        return "echo_pub_" + keyData.base64EncodedString().prefix(10)
    }
}

enum BlockchainClientError: Error, LocalizedError {
    case keyStorageFailed
    case walletNotLoaded
    case transactionFailed(String)
    case metadataRegistrationFailed(String)
    case unsupportedOperation(String)

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
}