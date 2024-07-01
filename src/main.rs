use tsa::Puzzle;

mod tsa;
mod chain_server;
mod wallet_server;
fn main() {
    let tasks=vec![100,200,200,300,400,300,400,300,400,400,800,100,400,500,700,800,400,300,100,100,200];
    let vms=vec![10,20,10,10,30,40];
    let best_solution=tsa::do_tsa(Puzzle::new(tasks, vms, 0.0));
    match best_solution {
        Some(s)=>{
            s.print();
        },
        None=>{
            println!("woops");
        }
    }
}
