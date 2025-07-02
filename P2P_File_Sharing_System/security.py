"""
security.py - Peer authentication and encryption utilities for EchoChain P2P node.

Features:
- Public/private key generation and loading (Ed25519)
- Challenge signing and verification
- ECDH session key negotiation
- AES-GCM encryption/decryption helpers
"""
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey, Ed25519PublicKey
from cryptography.hazmat.primitives.asymmetric.x25519 import X25519PrivateKey, X25519PublicKey
from cryptography.hazmat.primitives.kdf.hkdf import HKDF
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
import os

# --- Key Management ---
def generate_ed25519_keypair():
    """Generate a new Ed25519 key pair."""
    private_key = Ed25519PrivateKey.generate()
    public_key = private_key.public_key()
    return private_key, public_key

def save_private_key(private_key, filepath, password=None):
    """Save a private key to a file (PEM format, optional password)."""
    enc = serialization.BestAvailableEncryption(password.encode()) if password else serialization.NoEncryption()
    with open(filepath, 'wb') as f:
        f.write(private_key.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=enc
        ))

def load_private_key(filepath, password=None):
    """Load a private key from a file (PEM format, optional password)."""
    with open(filepath, 'rb') as f:
        return serialization.load_pem_private_key(f.read(), password=password.encode() if password else None)

def public_key_bytes(public_key):
    """Return the raw bytes of a public key."""
    return public_key.public_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PublicFormat.Raw
    )

def load_public_key(data):
    """Load a public key from raw bytes."""
    return Ed25519PublicKey.from_public_bytes(data)

# --- Challenge Signing/Verification ---
def sign_challenge(private_key, challenge: bytes) -> bytes:
    """Sign a challenge with the private key."""
    return private_key.sign(challenge)

def verify_challenge(public_key, challenge: bytes, signature: bytes) -> bool:
    """Verify a challenge signature with the public key."""
    try:
        public_key.verify(signature, challenge)
        return True
    except Exception:
        return False

# --- ECDH Session Key Negotiation ---
def generate_x25519_keypair():
    """Generate a new X25519 key pair for ECDH."""
    private_key = X25519PrivateKey.generate()
    public_key = private_key.public_key()
    return private_key, public_key

def derive_shared_key(private_key, peer_public_key_bytes):
    """Derive a shared secret using ECDH and HKDF."""
    peer_public_key = X25519PublicKey.from_public_bytes(peer_public_key_bytes)
    shared_secret = private_key.exchange(peer_public_key)
    # Derive a symmetric key using HKDF
    hkdf = HKDF(
        algorithm=hashes.SHA256(),
        length=32,
        salt=None,
        info=b'echochain-session-key',
    )
    return hkdf.derive(shared_secret)

# --- AES-GCM Encryption/Decryption ---
def encrypt_message(key: bytes, plaintext: bytes, associated_data: bytes = b'') -> (bytes, bytes):
    """Encrypt a message using AES-GCM. Returns (nonce, ciphertext)."""
    aesgcm = AESGCM(key)
    nonce = os.urandom(12)
    ciphertext = aesgcm.encrypt(nonce, plaintext, associated_data)
    return nonce, ciphertext

def decrypt_message(key: bytes, nonce: bytes, ciphertext: bytes, associated_data: bytes = b'') -> bytes:
    """Decrypt a message using AES-GCM."""
    aesgcm = AESGCM(key)
    return aesgcm.decrypt(nonce, ciphertext, associated_data) 