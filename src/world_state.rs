use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountState {
    balance: u64,

    // The nonce is the number of transactions sent from the account.
    // Each time tx is send, a the nonce increases by 1
    nonce: u64,
}

impl AccountState {
    fn new(balance: u64) -> Self {
        Self { nonce: 0, balance }
    }

    fn hash(&self) {
        // 1. serialize
        // 2. hash
    }
}

/* -- -- -- -- -- -- -- -- -- -- -- */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldState {
    /*
        In a real-world Ethereum client, Merkle Patricia trees (a variant of Merkle trees) are used
        for the world state to enable efficient and secure verification of data. They allow any
        change in the tree to be quickly detected and provide a way to easily prove that a specific
        transaction or state change occurred.

        In first iteration, I'll use a simpler data structure for the world state. I'll store the
        world state in a simple BTreeMap, where the key is the address of the account and the value
        is the AccountState struct.
    */
    accounts: BTreeMap<String, AccountState>,
}

impl WorldState {
    pub fn new() -> Self {
        let total_supply = 1000;
        let genesis_account = AccountState::new(total_supply);

        let mut accounts = BTreeMap::new();

        accounts.insert(ZERO_ADDRESS.to_string(), genesis_account);

        // println!("{:?}", accounts);

        WorldState { accounts }
    }
}
