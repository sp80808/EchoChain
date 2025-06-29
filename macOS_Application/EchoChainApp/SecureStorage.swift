import Foundation
import Security
import CryptoKit

class SecureStorage {
    private let keyTag = "com.echocain.secureenclave.key".data(using: .utf8)!
    
    // Generate and store a new Secure Enclave key
    func generateKeyPair() -> SecKey? {
        let attributes: [String: Any] = [
            kSecAttrKeyType as String: kSecAttrKeyTypeECSECPrimeRandom,
            kSecAttrKeySizeInBits as String: 256,
            kSecAttrTokenID as String: kSecAttrTokenIDSecureEnclave,
            kSecPrivateKeyAttrs as String: [
                kSecAttrIsPermanent as String: true,
                kSecAttrApplicationTag as String: keyTag
            ]
        ]
        
        var error: Unmanaged<CFError>?
        guard let privateKey = SecKeyCreateRandomKey(attributes as CFDictionary, &error) else {
            print("Error generating key: \(error?.takeRetainedValue().localizedDescription ?? "Unknown error")")
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
            kSecReturnRef as String: true
        ]
        
        var item: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        guard status == errSecSuccess else {
            print("Error retrieving key: \(SecCopyErrorMessageString(status, nil) as String? ?? "Unknown error")")
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
            .ecdsaSignatureMessageX962SHA256,
            data as CFData,
            &error
        ) as Data? else {
            print("Signing failed: \(error?.takeRetainedValue().localizedDescription ?? "Unknown error")")
            return nil
        }
        return signature
    }
    
    // Verify signature with public key
    func verify(signature: Data, data: Data, publicKey: SecKey) -> Bool {
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
            return false
        }
        return true
    }
}
