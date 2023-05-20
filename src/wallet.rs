use bincode::serialize;
use libsecp256k1::{sign, Message, PublicKey, SecretKey};
use rand::rngs::OsRng;
use tiny_keccak::{Hasher, Sha3};

use crate::transaction::{RawTransactionData, Transaction};
use crate::types::Result;

// The public key is generated from the private key using the Elliptic Curve Digital Signature
// Algorithm. You get a public address for your account by taking the last
// 20 bytes of the Keccak-256 hash of the public key and adding 0x to the beginning.

// You need a private key to sign transactions which output a signature.
// Others can then take the signature to derive your public key, proving the author of the message.
// This prevents malicious actors from broadcasting fake transactions because you can always verify
// the sender of a transaction.

// ❗️TODO: не до конца ясно:
// как из 20 байт публичного ключа можно проверить подлинность авторства?
// это же не весь публичный ключ целиком

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// TOOD: move to utils?
fn create_hash(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3::v256();
    hasher.update(bytes);

    let mut output: [u8; 32] = [0u8; 32];
    hasher.finalize(&mut output);

    output
}

pub struct KeyPair {
    pub private: SecretKey,
    pub public: [u8; 64],
}

pub struct Wallet {
    pub key_pair: KeyPair,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let key_pair = Wallet::generate_keypair();

        let public_key_hash: [u8; 32] = create_hash(&key_pair.public);

        let mut address_bytes: [u8; 20] = [0; 20];
        address_bytes.copy_from_slice(&public_key_hash[public_key_hash.len() - 20..]);

        let address_hex = hex::encode(address_bytes);

        let mut address: String = "0x".to_owned();
        address.push_str(&address_hex);

        Wallet { key_pair, address }
    }

    fn generate_keypair() -> KeyPair {
        let mut rng = OsRng::default();

        let private_key = SecretKey::random(&mut rng);
        let public_key = PublicKey::from_secret_key(&private_key);

        let public_key_bytes: [u8; 65] = public_key.serialize();

        let mut public_key_no_prefix: [u8; 64] = [0; 64];
        public_key_no_prefix.copy_from_slice(&public_key_bytes[1..]);

        KeyPair {
            private: private_key,
            public: public_key_no_prefix,
        }
    }

    pub fn create_transaction(&self, to_address: String, value: u64) -> Result<Transaction> {
        // FIXME
        let nonce = 0;

        let raw_tx_data = RawTransactionData {
            nonce,
            to: to_address.clone(),
            value,
        };

        let bytes = serialize(&raw_tx_data)?;
        let hash = create_hash(bytes.as_slice());

        let message = Message::parse_slice(&hash)?;

        let (signature, recid) = sign(&message, &self.key_pair.private);

        Ok(Transaction {
            nonce,
            to: to_address,
            value,
            hash,
            v: recid.serialize(),
            r: signature.r.b32(),
            s: signature.s.b32(),
        })
    }

    fn broadcast_transaction() {}

    // TODO: should accept world state as argument?
    fn check_account_balance() {}
}
