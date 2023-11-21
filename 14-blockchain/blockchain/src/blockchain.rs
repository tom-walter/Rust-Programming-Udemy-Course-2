extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use self::sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transaction: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        chain.generate_new_block();
        return chain
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header, count: 0, transactions: vec![],
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.current_transaction);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        println!("was added to chain");
        return true
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.current_transaction.push(
            Transaction { sender, receiver, amount}
        );
        return true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            // create a string of 64 zeros
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        return true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        return true
    }

    pub fn get_merkle(current_transaction: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();
        for trans in current_transaction {
            let hash = Chain::hash(&trans);
            merkle.push(hash);
        }
        if merkle.len() % 2 == 1 {
            let last = merkle.last().clone().unwrap();
            merkle.push(last.to_string());
        }

        while merkle.len () > 1 {
            let mut hash1 = merkle.remove(0);
            let mut hash2 = merkle.remove(0);
            hash1.push_str(&mut hash2);
            let new_hash = Chain::hash(&hash1);
            merkle.push(new_hash);
        }

        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice: &str = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val !=0 {header.nonce += 1;}
                    else {
                        println!("Block hash {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue
                }, 
            }
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let result = hasher.result();
        let vec_res = result.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut new_str = String::new();
        for hex in vec_res {
            write!(&mut new_str, "{:x}", hex).expect("unable to write hex to string");

        }
        return new_str
    }
}