pub mod block;
pub mod transaction;

use block::Block;
use transaction::Transaction;
pub struct BlockChain{
    transaction_pool:Vec<Transaction>,
    chain:Vec<Block>,
}