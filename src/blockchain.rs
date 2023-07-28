use bincode::serialize;
use serde::{Deserialize, Serialize};

use crate::utils::{create_hash, get_current_system_time};

use crate::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockHeader {
    parent_hash: String,
    timestamp: u64,
    nonce: u64,                // block nonce is used for mining
    world_state_hash: String, // should become world_state_root after migrating to Merkle Patricia tree
    transactions_hash: String, // should become transactions_root after migrating to Merkle Patricia tree
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    header: BlockHeader,

    /*
        Trie data structure is not needed except to verify blocks (and hence of course to mine them),
        and can technically be discarded once a block has been verified. However, it is implied that
        the transaction lists are stored locally in a trie, and serialized to lists to send to
        clients requesting the blockchain. Of course, that client will then reconstruct the transaction
        list trie for each block to verify the root hash
    */
    // TODO: implement as a trie
    transactions: Vec<Transaction>,
}

impl Block {
    fn new(parent_hash: String, state_hash: String, transactions_hash: String) {
        let timestamp = get_current_system_time();
    }

    fn create_genesis_block(world_state_hash: String) -> Self {
        let header = BlockHeader {
            parent_hash: hex::encode(&[0u8; 32]),
            world_state_hash,
            timestamp: get_current_system_time(),

            // skipping mining for now and just incrementing nonce
            nonce: 0,

            // TODO: genesis block may have transaction for initial supply
            transactions_hash: hex::encode(&[0u8; 32]),
        };

        Block {
            header,
            transactions: vec![],
        }
    }

    // fn get_hash(&self) -> String {
    //     let block_bytes = serialize(&self).unwrap();
    //     let block_hash = create_hash(&block_bytes);

    //     hex::encode(block_hash)
    // }
}

/* -- -- -- -- -- -- -- -- -- -- -- */

#[derive(Serialize, Deserialize, Debug, Clone)]
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
