pub mod block;
pub mod transaction;

use block::Block;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Receiver;
use transaction::Transaction;
#[derive(Serialize,Clone,Deserialize)]
pub struct BlockChain {
    pub transaction_pool: Vec<Transaction>,
    pub reward_pool: Vec<Transaction>,
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let transaction_pool: Vec<Transaction> = Vec::new();
        let reward_pool: Vec<Transaction> = Vec::new();
        let chain: Vec<Block> = Vec::new();
        let first_block = Block::new(0, Vec::new(), Vec::new());
        let mut output = Self {
            transaction_pool: transaction_pool,
            reward_pool: reward_pool,
            chain: chain,
        };
        output.chain.push(first_block);
        return output;
    }

    pub fn verify_and_add_transaction(&mut self, transaction: Transaction) -> bool {
        if !transaction.verify_self() {
            return false;
        }
        self.transaction_pool.push(transaction);
        return true;
    }

    pub async fn mine(
        &mut self,
        max_unit_num: usize,
        max_iteration_times: usize,
        max_loop_times:usize,
        min_block_len:usize,
        addr: String,
        receiver: &mut Receiver<bool>
    ) -> i32 {
        if self.transaction_pool.len()==0{
            println!("no transaction");
            return -1;
        }
        let mut succeeded: Vec<usize> = Vec::new();
        let mut count:usize=0;
        loop {
            println!("mine started");
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..self.transaction_pool.len());
            if succeeded.contains(&index){
                continue;
            }
            let mut transaction = self.transaction_pool[index].clone();
            let mine_result = transaction.mine(
                max_unit_num,
                max_iteration_times,
                addr.clone(),
                receiver
            ).await;
            if mine_result == 1 {
                succeeded.push(index);
                self.reward_pool.push(transaction);
            }else if mine_result==0{
            }else if mine_result==-1{
                for i in &succeeded {
                    self.transaction_pool.remove(*i);
                }
                println!("mine interupted");
                return -1;
            }

            if self.reward_pool.len()>=min_block_len{
                for i in &succeeded {
                    self.transaction_pool.remove(*i);
                }
                self.add_block();
                self.print();
                println!("mine done");
                return 1;
            }
            if succeeded.len()==self.transaction_pool.len()||count>max_loop_times{
                self.transaction_pool.clear();
                println!("transaction pool clear,mine not complete");
                return 0;
            }
            count+=1;
        }
    }

    pub fn add_block(&mut self) {
        let new_block = Block::new(
            self.chain.len(),
            self.chain.last().unwrap().hash_self(),
            self.reward_pool.clone()
        );
        self.reward_pool.clear();
        self.chain.push(new_block);
    }

    pub fn get_last_block(&self)->&Block{
        return self.chain.last().clone().unwrap();
    }

    pub fn print(&self) {
        println!("\n=======Transaction Pool=========");
        for transaction in &self.transaction_pool {
            transaction.print();
        }
        println!("==========Reward Pool=========");
        for transacion in &self.reward_pool {
            transacion.print();
        }
        println!("============Chain===========");
        for block in &self.chain {
            block.print();
        }
        println!("\n");
    }
}
