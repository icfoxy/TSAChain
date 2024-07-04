pub mod block;
pub mod transaction;

use block::Block;
use transaction::Transaction;
pub struct BlockChain{
    transaction_pool:Vec<Transaction>,
    reward_pool:Vec<Transaction>,
    chain:Vec<Block>,
}

impl BlockChain {
    pub fn new()->Self{
        let transaction_pool:Vec<Transaction>=Vec::new();
        let reward_pool:Vec<Transaction>=Vec::new();
        let chain:Vec<Block>=Vec::new();
        let first_block=Block::new(0, Vec::new(), Vec::new());
        let mut output= Self{
            transaction_pool:transaction_pool,
            reward_pool:reward_pool,
            chain:chain,
        };
        output.chain.push(first_block);
        return output;
    }

    pub fn verify_and_add_transaction(&mut self,transaction:Transaction)->bool{
        if !transaction.verify_self(){
            return false;
        }
        self.transaction_pool.push(transaction);
        return true;
    }

    pub fn mine(&mut self,max_unit_num:usize,max_iteration_times:usize,addr:String)->usize{
        let mut succeeded:Vec<usize>=Vec::new();
        for (i,transaction) in &mut self.transaction_pool.iter_mut().enumerate(){
            let mine_result=transaction.mine(max_unit_num, max_iteration_times, addr.clone());
            if mine_result{
                succeeded.push(i);
                self.reward_pool.push(transaction.clone());
            }
        }
        for i in &succeeded{
            self.transaction_pool.remove(*i);
        }
        return succeeded.len();
    }

    pub fn add_block(&mut self){
        let new_block=Block::new(self.chain.len(), self.chain.last().unwrap().hash_self(), self.reward_pool.clone());
        self.reward_pool.clear();
        self.chain.push(new_block);
    }

    pub fn print(&self){
        println!("\n=======Transaction Pool=========");
        for transaction in &self.transaction_pool{
            transaction.print();
        }
        println!("==========Reward Pool=========");
        for transacion in &self.reward_pool{
            transacion.print();
        };
        println!("============Chain===========");
        for block in &self.chain{
            block.print()
        }
        println!("\n");
    }
}