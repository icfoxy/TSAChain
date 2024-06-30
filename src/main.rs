mod tsa;
mod chain_server;
mod wallet_server;
fn main() {
    println!("Hello, world!");
    let su=tsa::Solution::new(&vec![10;10], &vec![100;10]);
    su.print();
}
