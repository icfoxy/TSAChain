pub mod tsa;
use serde::Serialize;
use tsa::{Puzzle,Solution};
#[derive(Clone,Serialize,Debug)]
pub struct Transaction{
    info:Info,
    signature:Vec<u8>,
}
#[derive(Clone,Serialize,Debug)]
pub struct Info{
    sender_addr:String,
    receiver_addr:String,
    puzzle:Puzzle,
    best_solution:Solution,
    value:i32,
}
impl Transaction {
    pub fn new(
        sender_addr:String,receiver_addr:String,puzzle:Puzzle,best_solution:Solution,value:i32,signature:Vec<u8>)->Self{
        return Transaction{
            info:Info{
                sender_addr:sender_addr,
                receiver_addr:receiver_addr,
                puzzle:puzzle,
                best_solution:best_solution,
                value:value,
            },
            signature:signature,
        }
    }
}