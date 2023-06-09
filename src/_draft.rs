use rand::rngs::OsRng;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// use ring::{rand, signature};
use libsecp256k1::{sign, PublicKey, SecretKey};
use tiny_keccak::{Hasher, Sha3};

// TODO: use anyhow for results ?
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

// TODO: how to implement readable user address? (like in Near)
// 0x8920R1N9... -> alice.near

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */
/* Ethereum root components */

// Ethereum is transaction-based “state” machine

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// TODO: refactor all components with modules

fn create_hash(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3::v256();
    hasher.update(bytes);

    let mut output: [u8; 32] = [0u8; 32];
    hasher.finalize(&mut output);

    output
}

struct KeyPair {
    pvt_key_hex: String, // в каком виде нужно хранить приватный ключи для подписей?
    pub_key_bytes: [u8; 64],
    pub_key_hex: String,
}

struct Wallet {
    // keypairs
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet::generate_keypair();

        Wallet {}
    }

    // -- Blockchain cryptgraphy --
    // 1. Hash functions
    // 2. Public key cryptography (asymmetric key cryptography):

    // Asymmetric cryptography is employed for signing transactions and verifying signatures,
    // while cryptographic hash functions are used to create unique hashes for each block and
    // link them together in the chain.

    fn generate_keypair() {
        let mut rng = OsRng::default();

        // Asymmetric cryptography is all about mathematical relation between private and public keys

        // ECDSA

        // Base point (G)
        // This is a predetermined point on the elliptic curve, specified by the chosen
        // elliptic curve standard (e.g., secp256k1 or secp256r1). The base point has a specific order 'n',
        // which is the number of times you can add the base point to itself before returning to the starting point.

        // Private key
        // A randomly generated large integer (256-bit) within a specific range of the
        // group of the elliptic curve. This group is generated by the base point G.
        let private_key = SecretKey::random(&mut rng); // scalar

        // Public key
        // Performing an elliptic curve multiplication (scalar multiplication) using the private key as
        // the scalar and the curve's base point as the point. The resulting point on the curve is public key.
        // Operation involves repeatedly adding a base point to itself a specific number of times (the private key).
        let public_key = PublicKey::from_secret_key(&private_key); // affine

        /* -- -- -- -- -- -- -- -- -- -- -- -- -- */

        let pvt_key_bytes = private_key.serialize(); // 32 bytes
        let pvt_key_hash = create_hash(&pvt_key_bytes); // 32 bytes
        let pvt_key_hex = hex::encode(pvt_key_hash); // 64 chars

        // PublicKey derived using scalar multiplication

        // let pub_key_bytes = pub_key.serialize(); // 65 bytes (uncompressed)

        // let pub_key_no_prefix: [u8; 64] = pub_key_bytes[1..];

        // let pub_key_hash = create_hash(&pub_key_no_prefix); // 32 bytes
        // let pub_key_hex = hex::encode(pub_key_hash); // 64 chars

        // println!("{:?}", pub_key_bytes);
        // println!("{:X?}", pub_key_bytes);
    }

    fn get_address() {}
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

struct Block {
    block_number: u64,
    timestamp: u64,
    hash: String, // 32 bytes (64 chars)
    previous_hash: String,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nBlock {{\n")?;
        write!(f, "    block_number: {},\n", self.block_number)?;
        write!(f, "    timestamp: {},\n", self.timestamp)?;
        write!(f, "    hash: {},\n", self.hash)?;
        write!(f, "    previous_hash: {},\n", self.previous_hash)?;
        write!(f, "}}")
    }
}

impl Block {
    pub fn create_genesis_block() -> Block {
        let genesis_hash = format!("{:0>64}", "");
        Block::new(0, &genesis_hash)
    }

    pub fn new(block_number: u64, previous_hash: &str) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Block {
            block_number,
            timestamp,
            hash: Block::calculate_hash(block_number, timestamp, previous_hash),
            previous_hash: previous_hash.to_string(),
        }
    }

    fn calculate_hash(block_number: u64, timestamp: u64, previous_hash: &str) -> String {
        let block_data = format!("{}{}{}", block_number, timestamp, previous_hash);
        let hash_bytes = create_hash(block_data.as_bytes());

        hex::encode(hash_bytes)
    }
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// Authenticate the sender: Digital signatures allow the network to verify that a transaction was
// created by the rightful owner of an account without revealing the private key.

// TODO: не до конца понимаю этот момент
// Ensure data integrity: Digital signatures help ensure that the transaction data has
// not been tampered with during transmission. Any alterations to the transaction data
// would cause the signature to become invalid.

// Provide non-repudiation: Once a transaction has been signed and broadcast to the network,
// the signer cannot deny having created and authorized it, since the signature is cryptographically linked to their private key.

// подпись транзы приватным ключем - это гарантия авторства
// любой может расшифровать подпись публичным ключем и убедиться в подлинности автора

// signature: encryption with private key
// Anyone with the public key can then decrypt and verify the message, which proves that
// it must have originated from the holder of the private key.

// A private key serves as a digital signature for its owner. When you sign a
// transaction with your private key, you create a unique signature that can be
// verified by others using your public key. This signature proves that you are
// the rightful owner of the digital assets associated with your public address,
// without having to reveal your private key itself.

// Signing the transaction: To authenticate and authorize the transaction,
// the sender signs it using their private key. This digital signature
// serves as proof that the transaction was created by the owner of the
// private key, without revealing the key itself.

// sender also includes a reference to previous transactions that
// prove they have enough balance in their account to make the transfer.

struct Transaction {
    id: String,
    // nonce: ?
    sender: String,
    recipient: String,
    // amount ?
    // fee ?
    signature: String,
}

impl Transaction {
    fn create() {}
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::create_genesis_block();

        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self) {
        if let Some(last_block) = self.blocks.last() {
            let new_block = Block::new(last_block.block_number + 1, &last_block.hash);
            self.blocks.push(new_block)
        }
    }

    pub fn validate_chain(&self) -> bool {
        self.blocks.len() <= 1
            || self.blocks.windows(2).all(|blocks| {
                let prev_block = &blocks[0];
                let block = &blocks[1];
                let hash =
                    Block::calculate_hash(block.block_number, block.timestamp, &prev_block.hash);

                hash == block.hash
            })
    }
}

fn rsa() {
    //
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block();
    blockchain.add_block();

    // let is_valid = blockchain.validate_chain();

    // println!("{:?}", blockchain);
    // println!("Is valid: {}", is_valid);

    // let wallet_1 = Wallet::new();

    // let plain_text = "Hey";
    // let bytes = plain_text.as_bytes();
    // let hex_string: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();

    // println!("{:?}", bytes); // [72, 101, 121]
    // println!("{:?}", hex_string); // ["48", "65", "79"]

    rsa();
}
