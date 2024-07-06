pub mod block_chain;
use std::sync::mpsc::Sender;

use block_chain::transaction::{ self, Transaction };
use hyper::{ body::to_bytes, Body, Error, Method, Request, Response, StatusCode };

use crate::{CHANNEL, MAIN_CHAIN};

pub async fn divide_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/chainServer/testAlive") => { test_alive_handler(req).await }
        (&Method::POST, "/chainServer/validAndPushTransaction") => {
            valid_and_push_transaction(req).await
        }
        _ => {
            let mut rep = Response::default();
            *rep.status_mut() = StatusCode::NOT_FOUND;
            Ok(rep)
        }
    }
}

pub async fn test_alive_handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Chain Server is alive")))
}

pub async fn valid_and_push_transaction(req: Request<Body>) -> Result<Response<Body>, Error> {
    println!("start to valid and push");
    let mut sender = CHANNEL.0.lock().unwrap().clone();
    println!("got sender");
    sender.send(false).await.unwrap();
    println!("sending done");
    let data = to_bytes(req.into_body()).await.unwrap();
    println!("to bytes done");
    let transaction:Transaction = serde_json::from_str(&String::from_utf8(data.to_vec()).unwrap()).unwrap();
    println!("convert done");
    let result = MAIN_CHAIN.lock().unwrap().verify_and_add_transaction(transaction);
    println!("push done");
    sender.send(true).await.unwrap();
    println!("pause complete");
    //TODO:广播
    Ok(Response::new(Body::from(result.to_string())))
}

//TODO:
// pub async fn update_chain(req: Request<Body>) -> Result<Response<Body>, Error> {}

// pub async fn update_transaction_pool(req: Request<Body>)-> Result<Response<Body>, Error>{}

// pub async fn ask_for_chain_update(req: Request<Body>)-> Result<Response<Body>, Error>{}
