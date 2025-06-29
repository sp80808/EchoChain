import Foundation
import Combine
import Security

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

// MARK: - Production Blockchain Client

class RealBlockchainClient: BlockchainClientProtocol {
    @Published var balance: Double = 0.0
    @Published var walletAddress: String = ""
    @Published var transactionHistory: [Transaction] = []

    private var privateKey: String?
    private let nodeURL = URL(string: "http://localhost:9933")! // Update to your node's URL
    private let keychainService = "com.echochain.wallet"
    private let keychainAccount = "privateKey"

    init() {
        // Attempt to load private key from Keychain
        if let storedKey = Self.loadPrivateKey(service: keychainService, account: keychainAccount) {
            self.privateKey = storedKey
            self.walletAddress = Self.deriveAddress(from: storedKey)
            Task {
                await self.fetchBalance()
                await self.fetchTransactionHistory()
            }
        }
    }

    @MainActor
    func createWallet() async throws {
        // TODO: Use real cryptography for keypair generation
        let newPrivateKey = UUID().uuidString // Replace with real key generation
        guard Self.savePrivateKey(newPrivateKey, service: keychainService, account: keychainAccount) else {
            throw BlockchainClientError.keyStorageFailed
        }
        self.privateKey = newPrivateKey
        self.walletAddress = Self.deriveAddress(from: newPrivateKey)
        // Optionally, fund the wallet or register on-chain
        await fetchBalance()
        await fetchTransactionHistory()
    }

    @MainActor
    func importWallet(privateKey: String) async throws {
        // TODO: Validate private key format
        guard Self.savePrivateKey(privateKey, service: keychainService, account: keychainAccount) else {
            throw BlockchainClientError.keyStorageFailed
        }
        self.privateKey = privateKey
        self.walletAddress = Self.deriveAddress(from: privateKey)
        await fetchBalance()
        await fetchTransactionHistory()
    }

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
        guard let privateKey = privateKey else { throw BlockchainClientError.walletNotLoaded }
        // TODO: Use real cryptography to sign
        let txData = "\(from)-\(to)-\(amount)-\(data ?? "")"
        let signed = "signed_\(txData)_with_\(privateKey.prefix(8))" // Replace with real signing
        return signed
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

    // MARK: - Keychain Helpers
    static func savePrivateKey(_ key: String, service: String, account: String) -> Bool {
        guard let data = key.data(using: .utf8) else { return false }
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account,
            kSecValueData as String: data
        ]
        SecItemDelete(query as CFDictionary)
        let status = SecItemAdd(query as CFDictionary, nil)
        return status == errSecSuccess
    }

    static func loadPrivateKey(service: String, account: String) -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        var dataTypeRef: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &dataTypeRef)
        guard status == errSecSuccess, let data = dataTypeRef as? Data else { return nil }
        return String(data: data, encoding: .utf8)
    }

    static func deriveAddress(from privateKey: String) -> String {
        // TODO: Use real address derivation
        return "echo" + privateKey.prefix(10) + "..." + privateKey.suffix(10)
    }

    /// Fetch transaction history from a SubQuery GraphQL endpoint
    /// - Parameters:
    ///   - account: The account address (hex string)
    ///   - endpoint: The GraphQL endpoint URL (e.g., http://localhost:3000)
    /// - Returns: Array of Transaction objects
    func fetchTransactionHistoryFromGraphQL(account: String, endpoint: URL) async throws -> [Transaction] {
        let query = """
        query {\n  transfers(filter: {from: {equalTo: \"\(account)\"}}) {\n    nodes {\n      id\n      from\n      to\n      amount\n      blockNumber\n      timestamp\n    }\n  }\n}\n"""
        let body: [String: Any] = ["query": query]
        let bodyData = try JSONSerialization.data(withJSONObject: body)
        var request = URLRequest(url: endpoint)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = bodyData

        let (data, response) = try await URLSession.shared.data(for: request)
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw BlockchainClientError.transactionFailed("Invalid response from indexer")
        }
        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        guard let dataDict = json?["data"] as? [String: Any],
              let transfers = dataDict["transfers"] as? [String: Any],
              let nodes = transfers["nodes"] as? [[String: Any]] else {
            throw BlockchainClientError.transactionFailed("Malformed indexer response")
        }
        let txs: [Transaction] = nodes.compactMap { node in
            guard let idStr = node["id"] as? String,
                  let id = UUID(uuidString: idStr),
                  let from = node["from"] as? String,
                  let to = node["to"] as? String,
                  let amountStr = node["amount"] as? String,
                  let amount = Double(amountStr),
                  let timestampStr = node["timestamp"] as? String,
                  let timestamp = ISO8601DateFormatter().date(from: timestampStr) else { return nil }
            return Transaction(
                id: id,
                type: from == account ? .sent : .received,
                amount: amount,
                from: from,
                to: to,
                date: timestamp
            )
        }
        return txs
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