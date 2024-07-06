use core::time;
use std::{
    convert::Infallible,
    net::SocketAddr,
    thread,
    sync::Mutex ,
    thread::sleep,
    time::Duration,
};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode };
use chain_server::block_chain::{
    transaction::{ self, tsa::{ Puzzle, Solution }, Transaction },
    BlockChain,
};
use hyper::service::{ make_service_fn, service_fn };
use wallet_server::wallet::{ self, Wallet };
use tokio::sync::{mpsc::{Receiver, Sender}, Barrier};
use std::sync::Arc;

mod chain_server;
mod wallet_server;

lazy_static::lazy_static! {
    static ref MAIN_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());
    static ref CHANNEL: (Mutex<Sender<bool>>, Mutex<Receiver<bool>>) = {
        let (sender, receiver) = tokio::sync::mpsc::channel::<bool>(1);
        (Mutex::new(sender), Mutex::new(receiver))
    };
}

async fn start_wallet_server() {
    let port = 9000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(|_conn: &hyper::server::conn::AddrStream| {
        async { Ok::<_, Infallible>(service_fn(wallet_server::divide_request)) }
    });
    println!("server starts at:{}", port);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn start_chain_server() {
    let port = 9001;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(|_conn: &hyper::server::conn::AddrStream| {
        async { Ok::<_, Infallible>(service_fn(chain_server::divide_request)) }
    });
    println!("server starts at:{}", port);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await{
        eprintln!("server error: {}", e);
    }
}

async fn start_mining(
    receiver: &mut Receiver<bool>,
) {
    println!("start mining...");
    loop {
        let result = {
            MAIN_CHAIN.lock()
            .unwrap()
            .mine(20, 100, 1000, 2, "m4".to_string(), receiver).await
        };
        if result==-1 {
            println!("waiting signal to resume...");
            receiver.recv().await;
            continue;
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

#[allow(dead_code)]
async fn pause_mining(sender: &Sender<bool>, barrier: Arc<Barrier>) {
    barrier.wait().await;
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!pause preparing..!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    sender.send(false).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    sender.send(true).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    sender.send(false).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    sender.send(true).await;
}

#[tokio::main]
async fn main() {
    let receiver=&mut CHANNEL.1.lock().unwrap();
    tokio::join!(start_chain_server(),start_wallet_server(),start_mining(receiver));
}
