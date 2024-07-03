use std::time::{SystemTime, UNIX_EPOCH};
use bincode::{self, serialize};
use ring::{digest::{digest, SHA256}, signature};
use serde::Serialize;

use super::transaction::Transaction;

#[derive(Clone,Serialize)]
pub struct Block{
    block_num:i32,
    pre_hash:Vec<u8>,
    transactions:Vec<Transaction>,
    timestamp:u128,
}
impl Block{
    pub fn new(block_num:i32,pre_hash:Vec<u8>,transactions:Vec<Transaction>)->Self{
        return Self{
            block_num:block_num,
            pre_hash:pre_hash,
            transactions:transactions,
            timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        }
    }
    pub fn hash_self(&mut self)->Vec<u8>{
        let converted=serialize(&self).unwrap();
        let output=digest(&SHA256,converted.as_ref());
        return output.as_ref().to_vec();
    }
    pub fn print(&self) {
        println!("Block {}{{\n  pre_hash: {:?},\n  transactions: {:?},\n  timestamp: {},\n}}", 
        self.block_num, self.pre_hash, self.transactions, self.timestamp);
    }
}