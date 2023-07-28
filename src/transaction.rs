use serde::{Deserialize, Serialize};

// Transactions within a block in Ethereum are processed sequentially, one after the other.
// Each transaction has the opportunity to modify the global state.

// Transaction Selection: The miner selects a set of valid transactions from the transaction pool.
// The specific selection algorithm can vary, but miners typically prioritize transactions that offer
// higher gas prices, as this maximizes their profit.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawTransactionData {
    // a sequentially incrementing counter which indicates the transaction number from the account
    pub nonce: u64,

    pub to: String,
    pub value: u64,
}

// When it comes to transaction processing, Ethereum transactions must be processed in order per account.
// For example, if an account has a current nonce of 10, the next valid transaction must have a nonce of 11.
// Transactions with a nonce less than 11 would be rejected as they have already been processed,
// and transactions with a nonce greater than 11 would be kept in the transaction pool until all preceding
// transactions have been processed. This ensures transactions from an account are processed in the order they were sent.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    // tx nonce match sender nonce
    pub nonce: u64,

    pub to: String,
    pub value: u64,
    pub hash: [u8; 32],

    pub v: u8,
    pub r: [u8; 32],
    pub s: [u8; 32],
}
