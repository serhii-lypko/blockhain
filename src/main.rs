mod transaction;
mod types;
mod wallet;

use bincode::serialize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// use transaction::Transaction;
use wallet::Wallet;

// TODO: use anyhow for results ?

const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

struct AccountState {
    balance: u64,
    // The nonce is the number of transactions sent from the account.
    // Each time tx is send, a the nonce increases by one
    nonce: u64,
    //
    // storage_root
    // code_hash
}

impl AccountState {
    fn new(balance: u64) -> Self {
        Self { nonce: 0, balance }
    }
}

/* -- -- -- -- -- -- -- -- -- -- -- */

struct WorldState {
    accounts: HashMap<String, AccountState>,
}

impl WorldState {
    fn new() -> Self {
        let total_supply = 1000;
        let genesis_account = AccountState::new(total_supply);

        let mut accounts: HashMap<String, AccountState> = HashMap::new();

        accounts.insert(ZERO_ADDRESS.to_string(), genesis_account);

        WorldState { accounts }
    }
}

/* -- -- -- -- -- -- -- -- -- -- -- */

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockHeader {
    parent_hash: String,
    timestamp: u64,
    // block nonce is used for mining
    nonce: u64,
    //
    // state_root: String,
    // transactions_root: String,
}

// TODO: how exactly hash block? need to hash transactions?
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    header: BlockHeader,
    // transactions: Vec<Transaction>,
}

impl Block {
    fn new() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    fn create_genesis_block() {}
}

/* -- -- -- -- -- -- -- -- -- -- -- */

struct Blockchain {}

impl Blockchain {
    fn new() -> Self {
        Blockchain {}
    }
}

/* -- -- -- -- -- -- -- -- -- -- -- */

// In summary, all miners are full nodes because they maintain a complete copy of the blockchain
// and validate transactions. However, not all full nodes are miners because they might not
// participate in the process of creating new blocks.
struct MinerNode {}

/* -- -- -- -- -- -- -- -- -- -- -- */

// TODO: how exactly nodes store all the data on their machines?
// do they use any internal databases or smth.

struct Node {
    blockchain: Blockchain,
    world_state: WorldState,
    //
    // when node receives new block it will filter it's mempool form txs from that new block
    // TODO: name is ok? maybe mempool?
    // transaction_pool: Vec<Transaction>,
    //
    // storage_state
    // network_interface
    // consensus
}

impl Node {
    // fn new() -> Self {
    //     let world_state = WorldState::new();
    //     let blockchain = Blockchain::new();

    //     Node {
    //         world_state,
    //         blockchain,
    //         transaction_pool: vec![],
    //     }
    // }

    fn validate_tx() {
        // checking that the transaction has a valid signature, the nonce is correct,
        // and the sender has enough Ether to cover the gas costs
    }

    fn validate_block() {}

    fn propagate_tx() {}

    fn propagate_block() {}

    fn execute_tx() {}
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// ❗️ TODO: чтобы не погрязнуть в сложных темах, нужно максимально декомпозировать задачи и идти по порядку❗️

// * Implementation plan *

// Part 1. wallet and transaction
// - ✅ create wallet: keypair, address
// - ✅ create transaction. generate it's hash
// - ✅ sign transaction

// Part 2. network basics
// - creating basic network
// - peer discovery
// -

// Part 3. minig and blocks creation
//

fn main() {
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    let alice_tx = alice_wallet.create_transaction(bob_wallet.address, 1000);

    println!("{:?}", alice_tx);

    // let tx_1 = Transaction {
    //     nonce: 0,
    //     to: String::from("0x0001"),
    // };

    // [0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 48, 120, 48, 48, 48, 49]
    // let tx_bytes = serialize(&tx_1).unwrap();
}
