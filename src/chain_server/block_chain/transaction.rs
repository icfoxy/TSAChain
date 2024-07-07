pub mod tsa;

use std::time::Duration;

use ring::digest::{ self, SHA256 };
use rsa::{ PaddingScheme, PublicKey };
use serde::{ Deserialize, Serialize };
use tokio::sync::mpsc::{error::TryRecvError, Receiver};
use tsa::{ do_tsa, Puzzle, Solution };

use crate::wallet_server::wallet::Wallet;
#[derive(Clone, Serialize, Debug, Deserialize,PartialEq, Eq)]
pub struct Transaction {
    pub info: Info,
    pub winner_addr: String,
    pub best_solution: Solution,
    pub sender_signature: Vec<u8>,
}
#[derive(Clone, Serialize, Debug, Deserialize,PartialEq, Eq)]
pub struct Info {
    pub sender_addr: String,
    pub sender_public_key: String,
    pub receiver_addr: String,
    pub puzzle: Puzzle,
    pub value: i32,
}
#[derive(Serialize, Deserialize)]
pub struct TransactionTransfer {
    pub sender_addr: String,
    pub sender_private_key: String,
    pub sender_public_key: String,
    pub receiver_addr: String,
    pub vms: Vec<i32>,
    pub tasks: Vec<i32>,
    pub expect: f32,
    pub value: i32,
    pub sign: String,
}

impl Transaction {
    #[allow(dead_code)]
    pub fn new(
        sender_addr: String,
        sender_public_key: String,
        receiver_addr: String,
        puzzle: Puzzle,
        value: i32
    ) -> Self {
        let best_solution = Solution::new_default(puzzle.vms.clone(), puzzle.tasks.clone());
        return Transaction {
            info: Info {
                sender_addr: sender_addr,
                sender_public_key: sender_public_key,
                receiver_addr: receiver_addr,
                puzzle: puzzle,
                value: value,
            },
            best_solution: best_solution,
            winner_addr: "none".to_string(),
            sender_signature: Vec::new(),
        };
    }
    pub fn new_from_transfer(transfer: &TransactionTransfer) -> Self {
        let puzzle = Puzzle::new(transfer.tasks.clone(), transfer.vms.clone(), transfer.expect);
        let best_solution = Solution::new_default(puzzle.vms.clone(), puzzle.tasks.clone());
        return Self {
            info: Info {
                sender_addr: transfer.sender_addr.clone(),
                sender_public_key: transfer.sender_public_key.clone(),
                receiver_addr: transfer.receiver_addr.clone(),
                puzzle: puzzle,
                value: transfer.value,
            },
            best_solution: best_solution,
            winner_addr: "none".to_string(),
            sender_signature: Vec::new(),
        };
    }
    pub fn print(&self) {
        println!("----------Transaction----------");
        println!("from:{}", self.info.sender_addr);
        println!("to:{}", self.info.receiver_addr);
        println!("value:{}", self.info.value);
        println!("winner addr:{}", self.winner_addr);
        // println!("signature:{:?}", Wallet::signature_to_string(&self.sender_signature));
        println!("signature:...");
        println!("solution:");
        self.best_solution.print();
    }

    pub fn verify_self(&self) -> bool {
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
        let sender_public_key = Wallet::string_to_public_key(&self.info.sender_public_key);
        let data = bincode::serialize(&self.info).unwrap();
        let hashed = digest::digest(&SHA256, &data);
        let verify_result = sender_public_key.verify(
            padding,
            hashed.as_ref(),
            &self.sender_signature
        );
        match verify_result {
            Ok(_) => { true }
            Err(_) => { false }
        }
    }

    pub async fn mine(
        &mut self,
        max_unit_num: usize,
        max_iteration_times: usize,
        addr: String,
        receiver: &mut Receiver<bool>
    ) -> i32 {
        let mut unit_num = 10;
        let mut iteration_times = 10;
        let original_time = self.best_solution.get_max_response_time();
        let mut my_solution:Solution;
        println!("original time:{}", original_time);
        let mut flag = true;
        while unit_num < max_unit_num && iteration_times < max_iteration_times {
            my_solution = do_tsa(&self.info.puzzle, unit_num, iteration_times).unwrap();
            if original_time * self.info.puzzle.expect > my_solution.get_max_response_time() {
                self.best_solution = my_solution;
                self.winner_addr = addr.clone();
                return 1;
            }
            if flag {
                unit_num *= 10;
                if unit_num > max_unit_num {
                    unit_num = max_unit_num;
                }
            } else {
                iteration_times *= 10;
                if iteration_times > max_iteration_times {
                    iteration_times = max_iteration_times;
                }
            }
            flag = !flag;
            match receiver.try_recv() {
                Ok(signal) => {
                    if !signal{
                        return -1;
                    }
                }
                Err(TryRecvError::Empty) => {
                    continue;  
                }
                Err(TryRecvError::Disconnected) => {
                    continue;
                }
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        return 0;
    }

}
