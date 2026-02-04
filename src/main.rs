use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug, Serialize, Clone)]
struct Block {
    index: u32,
    timestamp: u128,
    transactions: Vec<Transaction>, 
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let now = Utc::now().timestamp_millis() as u128;
        let mut block = Block {
            index,
            timestamp: now,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine_block();
        block
    }

    fn calculate_hash(&self) -> String {
        let tx_data = serde_json::to_string(&self.transactions).unwrap();
        let input = format!("{}{}{}{}{}", 
            self.index, self.timestamp, tx_data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }

    fn mine_block(&mut self) {
        let target = "00"; 
        while !self.hash.starts_with(target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("⛏️  Block mined: {}", self.hash);
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>, 
    mining_reward: f64,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            chain: vec![Block::new(0, vec![], "0".to_string())], 
            pending_transactions: vec![],
            mining_reward: 100.0, 
        }
    }

    fn create_transaction(&mut self, sender: String, receiver: String, amount: f64) {
        let new_tx = Transaction { sender, receiver, amount };
        self.pending_transactions.push(new_tx);
    }

    fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_tx = Transaction {
            sender: String::from("System"),
            receiver: miner_address,
            amount: self.mining_reward,
        };

        self.pending_transactions.push(reward_tx);

        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            self.pending_transactions.clone(), 
            last_block.hash.clone(),
        );

        self.chain.push(new_block);
        self.pending_transactions = vec![]; 
    }

    fn get_balance_of(&self, address: &str) -> f64 {
        let mut balance = 0.0;
        for block in &self.chain {
            for tx in &block.transactions {
                if tx.receiver == address {
                    balance += tx.amount;
                }
                if tx.sender == address {
                    balance -= tx.amount;
                }
            }
        }
        balance
    }
}

fn main() {
    let mut my_coin = Blockchain::new();

    println!("Creating transactions...");
    my_coin.create_transaction("Saad".to_string(), "Y".to_string(), 50.0);
    my_coin.create_transaction("Y".to_string(), "X".to_string(), 10.0);

    println!("Starting the Miner...");
    my_coin.mine_pending_transactions("Saad-Miner-Wallet".to_string());

    println!("\n--- Balance Check ---");
    println!("Saad's Balance: {}", my_coin.get_balance_of("Saad")); 
    
    println!("Y's Balance: {}", my_coin.get_balance_of("Y")); 
    
    println!("Miner's Balance: {}", my_coin.get_balance_of("Saad-Miner-Wallet")); 

    println!("\n--- Full Ledger ---");
    println!("{:#?}", my_coin.chain);
}