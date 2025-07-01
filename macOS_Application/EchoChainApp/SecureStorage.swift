import Foundation
import Security
import SubstrateKeychain
import LocalAuthentication // Import LocalAuthentication for Face ID/Touch ID

class SecureStorage {
    private let mnemonicTag = "com.echocain.wallet.mnemonic".data(using: .utf8)!
    
    // Generate a new Sr25519 keypair and store its mnemonic in Keychain with biometric authentication
    func generateMnemonicAndStore(requireBiometrics: Bool = false) async throws -> String {
        // Generate a new mnemonic for Sr25519 keypair
        let mnemonic = try Sr25519KeyPair.generateMnemonic()
        
        var query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: "EchoChainWalletMnemonic",
            kSecAttrService as String: "EchoChainApp",
            kSecAttrAccessible as String: kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly,
            kSecValueData as String: mnemonic.data(using: .utf8)!
        ]
        
        if requireBiometrics {
            let accessControl = SecAccessControlCreateWithFlags(
                kCFAllocatorDefault,
                .privateKeyUsage, // Use .privateKeyUsage for operations requiring private key access
                .userPresence // Require user presence (e.g., password, Touch ID, Face ID)
            )
            if let accessControl = accessControl {
                query[kSecAttrAccessControl as String] = accessControl
            } else {
                throw SecureStorageError.keyStorageFailed("Failed to create access control for biometrics.")
            }
        }
        
        SecItemDelete(query as CFDictionary) // Delete any existing item first
        let status = SecItemAdd(query as CFDictionary, nil)
        
        guard status == errSecSuccess else {
            throw SecureStorageError.keyStorageFailed("Failed to save mnemonic: \(SecCopyErrorMessageString(status, nil) as String? ?? "Unknown error")")
        }
        return mnemonic
    }
    
    // Retrieve the mnemonic from Keychain, potentially requiring biometric authentication
    func getMnemonic(requireBiometrics: Bool = false) async throws -> String? {
        var query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: "EchoChainWalletMnemonic",
            kSecAttrService as String: "EchoChainApp",
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        
        if requireBiometrics {
            query[kSecUseOperationPrompt as String] = "Authenticate to access your wallet."
        }
        
        var item: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        
        guard status == errSecSuccess else {
            if status == errSecItemNotFound {
                return nil // No mnemonic found
            }
            throw SecureStorageError.keyRetrievalFailed("Failed to retrieve mnemonic: \(SecCopyErrorMessageString(status, nil) as String? ?? "Unknown error")")
        }
        
        guard let data = item as? Data, let mnemonic = String(data: data, encoding: .utf8) else {
            throw SecureStorageError.keyRetrievalFailed("Failed to decode mnemonic data.")
        }
        return mnemonic
    }
    
    // Save an imported mnemonic to Keychain, potentially with biometric authentication
    func saveMnemonic(_ mnemonic: String, requireBiometrics: Bool = false) async throws {
        var query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: "EchoChainWalletMnemonic",
            kSecAttrService as String: "EchoChainApp",
            kSecAttrAccessible as String: kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly,
            kSecValueData as String: mnemonic.data(using: .utf8)!
        ]
        
        if requireBiometrics {
            let accessControl = SecAccessControlCreateWithFlags(
                kCFAllocatorDefault,
                .privateKeyUsage,
                .userPresence
            )
            if let accessControl = accessControl {
                query[kSecAttrAccessControl as String] = accessControl
            } else {
                throw SecureStorageError.keyStorageFailed("Failed to create access control for biometrics.")
            }
        }
        
        SecItemDelete(query as CFDictionary) // Delete any existing item first
        let status = SecItemAdd(query as CFDictionary, nil)
        
        guard status == errSecSuccess else {
            throw SecureStorageError.keyStorageFailed("Failed to save imported mnemonic: \(SecCopyErrorMessageString(status, nil) as String? ?? "Unknown error")")
        }
    }
    
    // Delete the mnemonic from Keychain
    func deleteMnemonic() throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: "EchoChainWalletMnemonic",
            kSecAttrService as String: "EchoChainApp"
        ]
        
        let status = SecItemDelete(query as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw SecureStorageError.keyDeletionFailed("Failed to delete mnemonic: \(SecCopyErrorMessageString(status, nil) as String? ?? "Unknown error")")
        }
    }
    
    // Check if biometrics are available
    func canEvaluatePolicy() -> Bool {
        return LAContext().canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: nil)
    }
}

enum SecureStorageError: Error, LocalizedError {
    case keyStorageFailed(String)
    case keyRetrievalFailed(String)
    case keyDeletionFailed(String)
    case biometricAuthenticationFailed(String)
    
    var errorDescription: String? {
        switch self {
        case .keyStorageFailed(let message):
            return "Secure storage failed: \(message)"
        case .keyRetrievalFailed(let message):
            return "Secure retrieval failed: \(message)"
        case .keyDeletionFailed(let message):
            return "Secure deletion failed: \(message)"
        case .biometricAuthenticationFailed(let message):
            return "Biometric authentication failed: \(message)"
        }
    }
}
