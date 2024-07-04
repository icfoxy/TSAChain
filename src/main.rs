use chain_server::block_chain::{transaction::{self, tsa::{Puzzle, Solution}, Transaction}, BlockChain};
use wallet_server::wallet::Wallet;


mod chain_server;
mod wallet_server;
fn main() {

    let mut chain=BlockChain::new();

    let tasks=vec![100,200,200,300,400,400,200,300,400,100];
    let vms=vec![10,20,40,30,10];
    let puzzle=Puzzle::new(tasks, vms, 0.2);
    let wallet=wallet_server::wallet::Wallet::new();
    // // let mut transaction=Transaction::new(
    // //     "sa".to_string(), "sk".to_string(), "r1".to_string(),puzzle, 22);
    // // let result=transaction.mine(200, 1000, "m1".to_string());
    // // println!("mine result:{}",result);
    // // transaction.print();
    let mut signed_transaction=wallet.create_signed_transaction(
        "s1".to_string(), "r1".to_string(), puzzle, 22);
    chain.verify_and_add_transaction(signed_transaction);
    chain.print();
    chain.mine(20, 100, wallet.addr);
    chain.print();
    chain.add_block();
    chain.print();
    // let mine_result=signed_transaction.mine(200, 1000, "m2".to_string());
    // let verify_result=signed_transaction.verify_self();
    // signed_transaction.print();
    // println!("v:{}\nm:{}",verify_result,mine_result);


    // println!("Hello world");
    // // wallet.print();
    // let mut transaction=Transaction::new("s1".to_string(),"pk".to_string(), "r1".to_string(), puzzle, 20);
    // wallet.sign_transaction(&mut transaction);
    // transaction.print();


    // let best_solution=tsa::do_tsa(&Puzzle::new(tasks, vms, 0.0), 20, 1000);
    // match best_solution {
    //     Some(s)=>{
    //         s.print();
    //     },
    //     None=>{
    //         println!("woops");
    //     }
    // }
}
