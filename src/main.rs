mod blockchain;
mod transaction;
mod types;
mod utils;
mod wallet;
mod world_state;

extern crate rmp_serde as rmps;
extern crate serde_derive;

use serde::{Deserialize, Serialize};

use blockchain::Blockchain;
use transaction::Transaction;
use wallet::Wallet;
use world_state::WorldState;

// TODO: use anyhow for results ?

/* = = = = = = = = = = = = = = = Implementation Plan = = = = = = = = = = = = = = = = =

❗️TODO: нужно максимально декомпозировать задачи и идти по порядку от простого к сложному❗️

(Part 1. single node)
1. implement basic flow for a single node with simplified world state and transactions storage
    - blockchain init (with genesis block)
    - world state init (with 0-account)
    - node init
    - create wallet
    - create transaction
    - execute transactions and update world state
    - consensus and PoW
2. all the same but with merkle patricia tree for world state and transactions storage

(Part 2. standalone node)
    - have separated running node which should store all the data on disk via key-value DB like RocksDB
    - have separated running wallet
    - they should be able to communicate with each other via (http?)

(Part 3. network of nodes)

= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = */

/*
    Documentation Agenda

    1. Document the Purpose of Each Component: Each file, function, class, or module should have a comment
    at the top explaining what it does and how it fits into the larger project. For complex functions,
    also consider including a brief explanation of the algorithm or logic used.

    2. Explain Your Design Decisions: If you made a choice between different possible implementations, explain
    why you chose the one you did. This can be very helpful for anyone reading the code later, including yourself.

    3. Describe the Data: If your code relies on specific data structures, explain what they are and why you're
    using them. This could be as simple as a comment explaining what each field in a struct is for, or it could
    be a more complex explanation of how different data structures interact.

    4. Explain Tricky Code: If there's a part of your code that's particularly complex or uses a non-obvious trick,
    leave a comment explaining it. If you had trouble writing it, someone else will probably have trouble understanding it.

    5. Comment on Protocol and Communication: In a decentralized system like Ethereum, it's important to document
    the protocols and communication mechanisms between different components of the system. This includes
    explaining any synchronization mechanisms, how data is transferred, how nodes join and leave the network, and so on.

    6. Keep the Documentation Updated: This is perhaps the most challenging part. When you make changes to your code,
    also update the corresponding comments and documentation. Outdated documentation can be more confusing than
    no documentation at all.
*/

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

/*
    Node in fact is a server and an interface for the blockchain.

    In summary, all miners are full nodes because they maintain a complete copy of the blockchain
    and validate transactions. However, not all full nodes are miners because they might not
    participate in the process of creating new blocks.

    TODO: separation for MinerNode, FullNode, LightNode etc.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Node {
    blockchain: Blockchain,
    world_state: WorldState,
    transaction_pool: Vec<Transaction>,
}

impl Node {
    fn new() -> Self {
        let world_state = WorldState::new();

        // TODO: should accept world_state_hash
        let blockchain = Blockchain::new();

        Node {
            blockchain,
            world_state,
            transaction_pool: vec![],
        }
    }

    fn validate_tx() {
        // checking that the transaction has a valid signature, the nonce is correct,
        // and the sender has enough Ether to cover the gas costs
    }

    fn validate_block() {}

    // that one suppose to be really interesting
    fn execute_tx() {}
}

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

fn main() {
    // let node = Node::new();

    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    let alice_tx = alice_wallet
        .create_transaction(bob_wallet.address, 1000)
        .unwrap();

    println!("{:?}", alice_tx);
}
