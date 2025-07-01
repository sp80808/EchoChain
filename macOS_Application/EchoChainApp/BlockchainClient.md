# For more information, see the main project README: ../../../README.md
# And the macOS Application README: ../README.md
# And the EchoChain Documentation and Development Plan: ../../../docs/EchoChain_Documentation_and_Development_Plan.md

# Production Blockchain Client for EchoChain macOS App

This document provides a production-ready Swift blockchain client stub, protocol, and integration notes for the EchoChain macOS app.

## BlockchainClientProtocol
```swift
import Foundation
import Combine

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
```

## RealBlockchainClient Stub
```swift
class RealBlockchainClient: BlockchainClientProtocol {
    @Published var balance: Double = 0.0
    @Published var walletAddress: String = ""
    @Published var transactionHistory: [Transaction] = []

    private var privateKey: String?
    private let nodeURL = URL(string: "http://localhost:9933")! // Replace with your node's URL

    func createWallet() async throws {
        // Generate a real keypair, store private key securely, derive address
        // Use CryptoKit for key generation and macOS Keychain for storage
        // Example:
        // let privateKey = Curve25519.Signing.PrivateKey()
        // Store privateKey in Keychain
        // Derive publicKey and address
        throw NSError(domain: "NotImplemented", code: 0)
    }

    func importWallet(privateKey: String) async throws {
        // Validate and store private key, derive address
        // Store in Keychain, derive publicKey and address
        throw NSError(domain: "NotImplemented", code: 0)
    }

    func fetchBalance() async throws {
        // Make JSON-RPC call to node to fetch balance for walletAddress
        // Use URLSession to POST to nodeURL
        throw NSError(domain: "NotImplemented", code: 0)
    }

    func fetchTransactionHistory() async throws {
        // Make JSON-RPC call to node to fetch transaction history
        // Use URLSession to POST to nodeURL
        throw NSError(domain: "NotImplemented", code: 0)
    }

    func signTransaction(from: String, to: String, amount: Double, data: String?) async throws -> String {
        // Sign transaction using privateKey from Keychain
        // Use CryptoKit for signing
        throw NSError(domain: "NotImplemented", code: 0)
    }

    func broadcastTransaction(signedTransaction: String) async throws -> String {
        // Broadcast signed transaction to node via JSON-RPC
        // Use URLSession to POST to nodeURL
        throw NSError(domain: "NotImplemented", code: 0)
    }

    func registerSampleMetadata(title: String, artist: String, p2pContentId: String, blockchainHash: String) async throws -> String {
        // Call smart contract or extrinsic to register sample metadata
        // Use URLSession to POST to nodeURL
        throw NSError(domain: "NotImplemented", code: 0)
    }
}
```

## Best Practices
- **Key Storage:** Use macOS Keychain for private keys, never UserDefaults.
- **Networking:** Use `URLSession` for JSON-RPC calls to your node.
- **Error Handling:** Surface errors to the UI and log for debugging.
- **Security:** Validate all inputs, sanitize data, and handle edge cases.

## Integration Notes
- Replace all `throw NSError(domain: "NotImplemented", code: 0)` with real logic.
- Inject `RealBlockchainClient` into your SwiftUI views.
- Test with your actual blockchain node and adapt endpoints as needed. 