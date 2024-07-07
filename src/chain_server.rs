pub mod block_chain;
use std::time::Duration;

use block_chain::{transaction::Transaction, BlockChain};
use hyper::{ body::to_bytes, Body, Client, Error, Method, Request, Response, StatusCode };
use tokio::time::timeout;

use crate::{CHANNEL, CONFIG, MAIN_CHAIN, SELF_NUM, UPDATE_LOCK};

pub async fn divide_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/chainServer/testAlive") => { test_alive_handler().await },
        (&Method::GET, "/chainServer/sayHi") => { say_hi().await },
        (&Method::GET, "/chainServer/getChain") => { get_chain().await },
        (&Method::POST, "/chainServer/receiveHi") => {receive_hi(req).await },
        (&Method::GET, "/chainServer/updateChain") => {update_chain().await },
        (&Method::POST, "/chainServer/receiveChainUpdate") => {receive_chain_update(req).await },
        (&Method::GET, "/chainServer/receiveTransactionBroadcast") => {receive_transaction_broadcast(req).await },
        (&Method::POST, "/chainServer/validAndPushTransaction") => {
            valid_and_push_transaction(req).await
        },
        _ => {
            let mut rep = Response::default();
            *rep.status_mut() = StatusCode::NOT_FOUND;
            Ok(rep)
        },
    }
}

pub async fn test_alive_handler()-> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Chain Server is alive")))
}

pub async fn valid_and_push_transaction(req: Request<Body>) -> Result<Response<Body>, Error> {
    println!("start to valid and push");
    let sender = CHANNEL.0.lock().unwrap().clone();
    sender.send(false).await.unwrap();
    let data = to_bytes(req.into_body()).await.unwrap();
    let transaction:Transaction = serde_json::from_str(&String::from_utf8(data.to_vec()).unwrap()).unwrap();
    let result = MAIN_CHAIN.lock().unwrap().verify_and_add_transaction(transaction.clone());
    //TODO:广播
    let len=CONFIG.lock().unwrap().nodes.len();
    for i in 0..len{
        if i==SELF_NUM.lock().unwrap().to_owned(){
            continue;
        }
        let url=CONFIG.lock().unwrap().nodes[i].to_string()+"/chainServer/receiveTransactionBroadcast";
        let client = Client::new();
        let json_string = serde_json::to_string(&transaction).unwrap();
        let req=Request::builder().uri(url).header("content-type", "application/json")
        .body(Body::from(json_string))
        .unwrap();
        let req_result=timeout(Duration::from_millis(200), client.request(req)).await;
        match req_result {
            Ok(_) => {
                println!("boradcast to node_{}",i);
            },
            Err(_) => {
                println!("broadcast:node_{} not reachable",i);
                continue;
            },
        }
    }
    sender.send(true).await.unwrap();
    println!("valid and push done");
    Ok(Response::new(Body::from(result.to_string())))
}

//实验
pub async fn say_hi() -> Result<Response<Body>, Error> {
    println!("starting hi...");
    let client = Client::new();
    let req=Request::builder().uri("http://localhost:9002/chainServer/receiveHi")
    .method("POST")
    .header("content-type", "application/json")
    .body(Body::from(serde_json::to_string("hi there").unwrap()))
    .unwrap();
    let resp=client.request(req).await.unwrap();
    println!("{}",resp.status());
    Ok(Response::new(Body::from("Hi done")))
}

pub async fn receive_hi(req: Request<Body>) -> Result<Response<Body>, Error>{
    println!("Hi received, processing...");
    let data = to_bytes(req.into_body()).await.unwrap();
    println!("{}",String::from_utf8(data.to_vec()).unwrap());
    Ok(Response::new(Body::from("Receive Hi done")))
}

//TODO:链共识还没做完
pub async fn update_chain() -> Result<Response<Body>, Error> {
    println!("!!!!!!!!!!!!!Updating MAINCHAIN...!!!!!!!!!!!!!!");
    let sender = CHANNEL.0.lock().unwrap().clone();
    sender.send(false).await.unwrap();
    let len=CONFIG.lock().unwrap().nodes.len();
    for i in 0..len{
        if i==SELF_NUM.lock().unwrap().to_owned(){
            continue;
        }
        let url=CONFIG.lock().unwrap().nodes[i].to_string()+"/chainServer/getChain";
        let client = Client::new();
        let req=Request::builder().uri(url).method("GET").body(Body::default()).unwrap();
        let req_result=timeout(Duration::from_secs(4), client.request(req)).await;
        let resp:Response<Body>;
        match req_result {
            Ok(v) => {
                resp=v.unwrap();
            },
            Err(_) => {
                println!("updating:node_{} not reachable",i);
                continue;
            },
        }
        let data = to_bytes(resp.into_body()).await.unwrap();
        let new_chain:BlockChain=serde_json::from_str(&String::from_utf8(data.to_vec()).unwrap()).unwrap();
        let len=MAIN_CHAIN.lock().unwrap().chain.len();
        if valid_chain(&new_chain)&&new_chain.chain.len()>=len{
            let last_block=new_chain.get_last_block();
            let mut main_chain=MAIN_CHAIN.lock().unwrap().to_owned();
            for transaction in &last_block.transactions{
                main_chain.transaction_pool.retain(|tx|{tx!=transaction});
                main_chain.reward_pool.retain(|tx|tx!=transaction);
            }
            main_chain.chain=new_chain.chain.clone();
            *MAIN_CHAIN.lock().unwrap()=main_chain.clone();
        }
    }
    sender.send(true).await.unwrap();
    println!("update MAINCHAIN complete");
    Ok(Response::new(Body::from("MAIN_CHAIN updated")))
}

pub async fn get_chain() -> Result<Response<Body>, Error>{
    println!("Getting chain");
    let sender = CHANNEL.0.lock().unwrap().clone();
    sender.send(false).await.unwrap();
    let json_string=serde_json::to_string(&MAIN_CHAIN.lock().unwrap().to_owned().clone()).unwrap();
    sender.send(true).await.unwrap();
    println!("Get_chain complete");
    Ok(Response::new(Body::from(json_string)))
}

//TODO:
pub fn valid_chain(new_chain:&BlockChain)->bool{
    if new_chain.chain.len()!=0{
        return true;
    }
    return false;
}

pub async fn receive_transaction_broadcast(req: Request<Body>)-> Result<Response<Body>, Error>{
    println!("start to valid and push");
    let sender = CHANNEL.0.lock().unwrap().clone();
    sender.send(false).await.unwrap();
    let data = to_bytes(req.into_body()).await.unwrap();
    let transaction:Transaction = serde_json::from_str(&String::from_utf8(data.to_vec()).unwrap()).unwrap();
    println!("start verify");
    let result = MAIN_CHAIN.lock().unwrap().verify_and_add_transaction(transaction);
    if !result{
        return Ok(Response::new(Body::from("not added")))
    }
    sender.send(true).await.unwrap();
    println!("broadcast valid and push complete");
    Ok(Response::new(Body::from("Transacton pool updated")))
}

pub async fn ask_for_chain_update(){
    if *UPDATE_LOCK.lock().unwrap(){
        *UPDATE_LOCK.lock().unwrap()=false;
        println!("crashed");
        return;
    }
    println!("!!!!!!!!!!!!Asking for chain update!!!!!!!!!!");
    let len=CONFIG.lock().unwrap().nodes.len();
    for i in 0..len{
        if i==SELF_NUM.lock().unwrap().to_owned(){
            continue;
        }
        let url=CONFIG.lock().unwrap().nodes[i].to_string()+"/chainServer/receiveChainUpdate";
        let client = Client::new();
        let json_string=serde_json::to_string(&MAIN_CHAIN.lock().unwrap().to_owned().clone()).unwrap();
        let req=Request::builder().uri(url)
        .header("content-type", "application/json")
        .method("POST").body(Body::from(json_string)).unwrap();
        let req_result=timeout(Duration::from_secs(2), client.request(req)).await;
        match req_result {
            Ok(_) => {
                println!("asking:node_{} ask done",i);
            },
            Err(_) => {
                println!("asking:node_{} not reachable",i);
                continue;
            },
        }
    }
}
pub async fn receive_chain_update(req: Request<Body>)-> Result<Response<Body>, Error>{
    *UPDATE_LOCK.lock().unwrap()=true;
    println!("Receiving chain");
    let sender = CHANNEL.0.lock().unwrap().clone();
    sender.send(false).await.unwrap();
    let data = to_bytes(req.into_body()).await.unwrap();
        let new_chain:BlockChain=serde_json::from_str(&String::from_utf8(data.to_vec()).unwrap()).unwrap();
        let len=MAIN_CHAIN.lock().unwrap().chain.len();
        if valid_chain(&new_chain)&&new_chain.chain.len()>=len{
            let last_block=new_chain.get_last_block();
            let mut main_chain=MAIN_CHAIN.lock().unwrap().to_owned();
            for transaction in &last_block.transactions{
                main_chain.transaction_pool.retain(|tx|{tx!=transaction});
                main_chain.reward_pool.retain(|tx|tx!=transaction);
            }
            main_chain.chain=new_chain.chain.clone();
            *MAIN_CHAIN.lock().unwrap()=main_chain.clone();
            println!("Updated chain:");
            MAIN_CHAIN.lock().unwrap().print();
        }else{
            println!("MAINCHAIN not updated");
        }
    sender.send(true).await.unwrap();
    Ok(Response::new(Body::from("MAINCHAIN updated")))
}   
