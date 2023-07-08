use bincode::serialize;
use serde::{Deserialize, Serialize};

use crate::utils::{create_hash, get_current_system_time};

use crate::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockHeader {
    parent_hash: String,
    timestamp: u64,
    nonce: u64,                // block nonce is used for mining
    state_hash: String,        // should become state_root after migrating to Merkle Patricia tree
    transactions_hash: String, // should become transactions_root after migrating to Merkle Patricia tree
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    header: BlockHeader,

    // storing transactions in simplified data structure instead of Merkle Patricia tree
    transactions: Vec<Transaction>,
}

impl Block {
    fn new() {
        let timestamp = get_current_system_time();
    }

    fn create_genesis_block(state_hash: String) -> Self {
        let header = BlockHeader {
            parent_hash: hex::encode(&[0u8; 32]),
            state_hash,
            timestamp: get_current_system_time(),
            nonce: 0,

            // TODO: genesis block may have transaction for initial supply
            transactions_hash: hex::encode(&[0u8; 32]),
        };

        Block {
            header,
            transactions: vec![],
        }
    }

    fn hash(&self) -> String {
        let block_bytes = serialize(&self).unwrap();
        let block_hash = create_hash(&block_bytes);

        hex::encode(block_hash)
    }
}

/* -- -- -- -- -- -- -- -- -- -- -- */

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::create_genesis_block(String::from("TODO"));

        let blocks = vec![genesis_block];

        Blockchain { blocks }
    }
}
