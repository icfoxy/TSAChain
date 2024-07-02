use tsa::{Puzzle, Solution};


mod tsa;
mod chain_server;
mod wallet_server;
fn main() {
    let tasks=vec![100,200,200,300,400,400,200,300,400,100,200,300,400,500,100,200];
    let vms=vec![10,20,40,30,10];
    let best_solution=tsa::do_tsa(&Puzzle::new(tasks, vms, 0.0), 20, 1000);
    match best_solution {
        Some(s)=>{
            s.print();
        },
        None=>{
            println!("woops");
        }
    }

    
}
