use std::time::{SystemTime, UNIX_EPOCH};

use tiny_keccak::{Hasher, Sha3};

// TODO: combine and hash transactions using merkle tree

// TODO: implement nice formatter for Block
#[derive(Debug)]
struct Block {
    block_number: u64,
    timestamp: u64,
    hash: String,
    previous_hash: String,
}

impl Block {
    pub fn new(block_number: u64, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Block {
            block_number,
            timestamp,
            hash: Block::calculate_hash(block_number, timestamp, previous_hash.clone()),
            previous_hash,
        }
    }

    fn calculate_hash(block_number: u64, timestamp: u64, previous_hash: String) -> String {
        let mut hasher = Sha3::v256();

        let hash_body = format!("{}{}{}", block_number, timestamp, previous_hash);

        hasher.update(hash_body.as_bytes());

        // 256-bit (32 byte) hash
        let mut output = [0u8; 32];

        hasher.finalize(&mut output);

        let hex = hex::encode(output);

        // println!("{:?}", output);
        // println!("{:?}", hex);

        // 64 characters long because each byte is represented by
        // two hexadecimal characters
        hex
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_hash = format!("{:0>64}", "");
        let genesis_block = Block::new(0, genesis_hash);

        println!("{:?}", genesis_block);

        Blockchain { blocks: vec![] }
    }
}

fn main() {
    let blockchain = Blockchain::new();

    // let block = Block::new("test");
    // let hash = Block::calculate_hash();
}
