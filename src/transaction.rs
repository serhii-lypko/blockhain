use serde::{Deserialize, Serialize};

// Transactions within a block in Ethereum are processed sequentially, one after the other.
// Each transaction has the opportunity to modify the global state.

// Transaction Selection: The miner selects a set of valid transactions from the transaction pool.
// The specific selection algorithm can vary, but miners typically prioritize transactions that offer
// higher gas prices, as this maximizes their profit.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawTransactionData {
    pub nonce: u64,
    pub to: String,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub nonce: u64, // tx nonce match sender nonce
    pub to: String,
    pub value: u64,
    pub hash: [u8; 32],
    pub v: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
}
