use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use tiny_keccak::{Hasher, Sha3};

// TODO: use anyhow for results ?
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

// TODO: create modules for Transaction, Block, Blockchain etc.

// TODO: combine and hash transactions using merkle tree

struct Block {
    block_number: u64,
    timestamp: u64,
    hash: String,
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
        let mut hasher = Sha3::v256();

        let hash_body = format!("{}{}{}", block_number, timestamp, previous_hash);

        hasher.update(hash_body.as_bytes());

        // 256-bit (32 byte) hash
        let mut output = [0u8; 32];

        hasher.finalize(&mut output);

        let hex = hex::encode(output);

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

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block();
    blockchain.add_block();

    let is_valid = blockchain.validate_chain();

    println!("{}", is_valid);
    // println!("{:?}", blockchain);

    // let block = Block::new("test");
    // let hash = Block::calculate_hash();
}
