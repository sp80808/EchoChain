import Foundation
import Security
import CryptoKit

class SecureStorage {
    private let keyTag = "com.echocain.secureenclave.key".data(using: .utf8)!
    
    // Generate and store a new Secure Enclave key
    func generateKeyPair() -> SecKey? {
        // TODO: Consider adding user authentication (e.g., Face ID/Touch ID) for key generation.
        let attributes: [String: Any] = [
            kSecAttrKeyType as String: kSecAttrKeyTypeECSECPrimeRandom,
            kSecAttrKeySizeInBits as String: 256,
            kSecAttrTokenID as String: kSecAttrTokenIDSecureEnclave, // Ensures key is in Secure Enclave
            kSecPrivateKeyAttrs as String: [
                kSecAttrIsPermanent as String: true,
                kSecAttrApplicationTag as String: keyTag,
                kSecAttrAccessControl as String: SecAccessControlCreateWithFlags(
                    kCFAllocatorDefault,
                    .privateKeyUsage, // TODO: Adjust access control flags as per security requirements (e.g., .userPresence, .biometryAny)
                    .privateKeyUsage,
                    nil
                )!
            ]
        ]
        
        var error: Unmanaged<CFError>?
        guard let privateKey = SecKeyCreateRandomKey(attributes as CFDictionary, &error) else {
            print("Error generating key: \(error?.takeRetainedValue().localizedDescription ?? "Unknown error")")
            // TODO: Log detailed error for debugging.
            return nil
        }
        return privateKey
    }
    
    // Retrieve the private key from Secure Enclave
    func getPrivateKey() -> SecKey? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassKey,
            kSecAttrApplicationTag as String: keyTag,
            kSecAttrKeyType as String: kSecAttrKeyTypeECSECPrimeRandom,
            kSecReturnRef as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne // Ensure only one key is returned
        ]
        
        var item: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        guard status == errSecSuccess else {
            print("Error retrieving key: \(SecCopyErrorMessageString(status, nil) as String? ?? "Unknown error")")
            // TODO: Handle specific error codes (e.g., errSecItemNotFound).
            return nil
        }
        return (item as! SecKey)
    }
    
    // Sign data using the private key
    func sign(data: Data) -> Data? {
        guard let privateKey = getPrivateKey() else { return nil }
        
        var error: Unmanaged<CFError>?
        guard let signature = SecKeyCreateSignature(
            privateKey,
            .ecdsaSignatureMessageX962SHA256, // TODO: Confirm the correct algorithm for blockchain signing.
            data as CFData,
            &error
        ) as Data? else {
            print("Signing failed: \(error?.takeRetainedValue().localizedDescription ?? "Unknown error")")
            // TODO: Log detailed error for debugging.
            return nil
        }
        return signature
    }
    
    // Verify signature with public key
    func verify(signature: Data, data: Data, publicKey: SecKey) -> Bool {
        // TODO: Implement robust verification, potentially using a blockchain-specific verification method.
        let status = SecKeyVerifySignature(
            publicKey,
            .ecdsaSignatureMessageX962SHA256,
            data as CFData,
            signature as CFData,
            nil
        )
        return status
    }
    
    // Get public key from private key
    func getPublicKey() -> SecKey? {
        guard let privateKey = getPrivateKey() else { return nil }
        return SecKeyCopyPublicKey(privateKey)
    }
    
    // Delete key pair
    func deleteKeyPair() -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassKey,
            kSecAttrApplicationTag as String: keyTag,
            kSecAttrKeyType as String: kSecAttrKeyTypeECSECPrimeRandom
        ]
        
        let status = SecItemDelete(query as CFDictionary)
        if status != errSecSuccess && status != errSecItemNotFound {
            print("Error deleting key: \(SecCopyErrorMessageString(status, nil) as String? ?? "Unknown error")")
            // TODO: Add user confirmation before deleting keys.
            return false
        }
        return true
    }
}
