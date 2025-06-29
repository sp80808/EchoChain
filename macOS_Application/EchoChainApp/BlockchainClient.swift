import Foundation
import Combine
import Security

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
}

// MARK: - Production Blockchain Client

class RealBlockchainClient: BlockchainClientProtocol {
    @Published var balance: Double = 0.0
    @Published var walletAddress: String = ""
    @Published var transactionHistory: [Transaction] = []

    private let nodeURL = URL(string: "http://localhost:9933")! // Update to your node's URL
    private let secureStorage = SecureStorage()

    init() {
        // Attempt to load private key from Secure Enclave
        if let publicKey = secureStorage.getPublicKey() {
            self.walletAddress = Self.deriveAddress(from: publicKey)
            Task {
                await self.fetchBalance()
                await self.fetchTransactionHistory()
            }
        }
    }

    @MainActor
    func createWallet() async throws {
        guard let privateKey = secureStorage.generateKeyPair() else {
            throw BlockchainClientError.keyStorageFailed
        }
        guard let publicKey = secureStorage.getPublicKey() else {
            throw BlockchainClientError.keyStorageFailed
        }
        self.walletAddress = Self.deriveAddress(from: publicKey)
        await fetchBalance()
        await fetchTransactionHistory()
    }

    // Temporarily commenting out importWallet due to Secure Enclave limitations
    /*
    @MainActor
    func importWallet(privateKey: String) async throws {
        // This functionality needs to be re-evaluated for Secure Enclave
        throw BlockchainClientError.unsupportedOperation("Importing raw private keys is not supported with Secure Enclave.")
    }
    */

    @MainActor
    func fetchBalance() async throws {
        guard !walletAddress.isEmpty else { throw BlockchainClientError.walletNotLoaded }
        let params = [walletAddress]
        let request = JSONRPCRequest(method: "chain_getBalance", params: params)
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
        // Construct the data to be signed (e.g., a hash of the transaction details)
        let transactionDetails = "\(from)\(to)\(amount)\(data ?? "")"
        guard let dataToSign = transactionDetails.data(using: .utf8) else {
            throw BlockchainClientError.transactionFailed("Failed to encode transaction data for signing.")
        }

        guard let signature = secureStorage.sign(data: dataToSign) else {
            throw BlockchainClientError.transactionFailed("Failed to sign transaction.")
        }
        // For now, return a base64 encoded signature. In a real scenario, this would be part of a signed transaction object.
        return signature.base64EncodedString()
    }

    @MainActor
    func broadcastTransaction(signedTransaction: String) async throws -> String {
        let params = [signedTransaction]
        let request = JSONRPCRequest(method: "chain_broadcastTransaction", params: params)
        let result: String = try await sendRPC(request: request)
        return result // tx hash
    }

    @MainActor
    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String {
        guard let privateKey = privateKey else { throw BlockchainClientError.walletNotLoaded }
        let params = [walletAddress, title, artist, p2pContentId, blockchainHash]
        let request = JSONRPCRequest(method: "chain_registerSample", params: params)
        let result: String = try await sendRPC(request: request)
        return result // registration tx hash
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