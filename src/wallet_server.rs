pub mod wallet;
use hyper::{ body::to_bytes, Body, Error, Method, Request, Response, StatusCode };
use wallet::Wallet;
use serde_json::json;

use crate::chain_server::block_chain::transaction::{ self, Transaction };
pub async fn divide_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/walletServer/testAlive") => { test_alive_handler(req).await }
        (&Method::GET, "/walletServer/createWallet") => { create_wallet(req).await }
        (&Method::POST, "/walletServer/signTransaction") => { sign_transaction(req).await }
        _ => {
            let mut resp = Response::default();
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}

pub async fn test_alive_handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Wallet Server is alive")))
}

pub async fn create_wallet(req: Request<Body>) -> Result<Response<Body>, Error> {
    println!("creating wallet...");
    let wallet = Wallet::new();
    let json_string = serde_json::to_string(&wallet.to_transfer()).unwrap();
    Ok(Response::new(Body::from(json_string)))
}

pub async fn sign_transaction(req: Request<Body>) -> Result<Response<Body>, Error> {
    let data = to_bytes(req.into_body()).await.unwrap();
    let transfer = serde_json::from_str(&String::from_utf8(data.to_vec()).unwrap()).unwrap();
    let mut transaction = Transaction::new_from_transfer(&transfer);
    let wallet = Wallet::new_from_private_key_str(&transfer.sender_private_key);
    wallet.sign_transaction(&mut transaction);
    let json_string = serde_json::to_string(&transaction).unwrap();
    Ok(Response::new(Body::from(json_string)))
}
