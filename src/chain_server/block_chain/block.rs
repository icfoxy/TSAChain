use std::time::{ SystemTime, UNIX_EPOCH };
use bincode::{ self, serialize };
use ring::digest::{ digest, SHA256 };
use serde::{Deserialize, Serialize};

use super::transaction::Transaction;

#[derive(Clone, Serialize,Deserialize)]
pub struct Block {
    block_num: usize,
    pre_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub timestamp: u128,
}
impl Block {
    pub fn new(block_num: usize, pre_hash: Vec<u8>, transactions: Vec<Transaction>) -> Self {
        return Self {
            block_num: block_num,
            pre_hash: pre_hash,
            transactions: transactions,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        };
    }
    pub fn hash_self(&self) -> Vec<u8> {
        let converted = serialize(&self).unwrap();
        let output = digest(&SHA256, converted.as_ref());
        return output.as_ref().to_vec();
    }
    pub fn print(&self) {
        println!(
            "Block {}{{\n  pre_hash: {:?},\n timestamp: {},\n}}",
            self.block_num,
            self.pre_hash,
            self.timestamp
        );
        for transaction in &self.transactions {
            transaction.print();
        }
    }
}
