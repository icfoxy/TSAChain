use core::time;
use std::{
    convert::Infallible, fs::File, net::SocketAddr, sync::Mutex, thread::{self, sleep}, time::Duration
};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode };
use chain_server::{ask_for_chain_update, block_chain::{
    transaction::{ self, tsa::{ Puzzle, Solution }, Transaction },
    BlockChain,
}};
use hyper::service::{ make_service_fn, service_fn };
use serde::Deserialize;
use wallet_server::wallet::{ self, Wallet };
use tokio::sync::{mpsc::{Receiver, Sender}, Barrier};
use std::sync::Arc;
use std::io::Read;

mod chain_server;
mod wallet_server;
use serde_yaml;

lazy_static::lazy_static! {
    static ref MAIN_CHAIN: Mutex<BlockChain> = Mutex::new(BlockChain::new());
    static ref CHANNEL: (Mutex<Sender<bool>>, Mutex<Receiver<bool>>) = {
        let (sender, receiver) = tokio::sync::mpsc::channel::<bool>(1);
        (Mutex::new(sender), Mutex::new(receiver))
    };
    static ref CONFIG: Mutex<Config> = {
        let mut file = File::open("resources/application.yml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: Config = serde_yaml::from_str(&contents).unwrap();
        Mutex::new(config)
    };
    static ref SELF_NUM:Mutex<usize>=Mutex::new(0);
    static ref UPDATE_LOCK:Mutex<bool>=Mutex::new(false);
}

#[derive(Debug, Deserialize)]
struct Node{
    addr:[u8;4],
    port:u16,
}
impl Node {
    pub fn to_string(&self)->String{
        format!("http://{}.{}.{}.{}:{}", self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.port)
    }
}
#[derive(Debug, Deserialize)]
struct Config{
    pub nodes:Vec<Node>,
}

async fn start_wallet_server(addr:[u8;4],port:u16) {
    let url = SocketAddr::from((addr,port));

    let make_svc = make_service_fn(|_conn: &hyper::server::conn::AddrStream| {
        async { Ok::<_, Infallible>(service_fn(wallet_server::divide_request)) }
    });
    println!("wallet server starts at:{}", port);
    let server = Server::bind(&url).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn start_chain_server(addr:[u8;4],port:u16) {
    let url = SocketAddr::from((addr,port));

    let make_svc = make_service_fn(|_conn: &hyper::server::conn::AddrStream| {
        async { Ok::<_, Infallible>(service_fn(chain_server::divide_request)) }
    });
    println!("chain server starts at:{}", port);
    let server = Server::bind(&url).serve(make_svc);

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
            .mine(20, 100, 1000, 2, "miner ".to_string()+&SELF_NUM.lock().unwrap().to_string(), receiver).await
        };
        if result==-1 {
            println!("waiting signal to resume...");
            receiver.recv().await;
            continue;
        }
        else if result==1{
            ask_for_chain_update().await;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
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
    let mut node_num:usize=0;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(value) => {
                node_num=value;
                println!("starting: node_{}...", value);
            },
            Err(_) => println!("无法将命令行参数转换为 usize"),
        }
    }else{
        println!("未检测到node_num,正确启动方式：cargo run node_num");
        return;
    }
    *SELF_NUM.lock().unwrap()=node_num;
    let addr=CONFIG.lock().unwrap().nodes[node_num].addr.clone();
    let port=CONFIG.lock().unwrap().nodes[node_num].port;
    let receiver=&mut CHANNEL.1.lock().unwrap();
    println!("server starts at {:?}:{}",addr,port);
    tokio::join!(start_chain_server(addr.clone(),port),start_wallet_server(addr.clone(),port+1),start_mining(receiver));
}
