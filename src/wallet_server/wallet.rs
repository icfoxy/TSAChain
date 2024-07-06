use pkcs1::FromRsaPublicKey;
use ring::digest::{self, SHA256};
use rsa::{pkcs1::FromRsaPrivateKey, PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use serde::Serialize;
use sha2::{Sha256, Digest};
use hex;

use crate::chain_server::block_chain::transaction::{ tsa::Puzzle, Transaction};
pub struct Wallet {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
    pub addr: String,
}

#[derive(Serialize)]
pub struct WalletTransfer{
    pub private_key: String,
    pub public_key: String,
    pub addr: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to create key");
        let public_key = RsaPublicKey::from(&private_key);
        let addr = Wallet::public_key_to_addr(&public_key);
        Wallet {
            private_key,
            public_key,
            addr,
        }
    }

    pub fn new_from_private_key_str(private_key_str:&String)->Self{
        let private_key=Wallet::string_to_private_key(&private_key_str);
        let public_key = RsaPublicKey::from(&private_key);
        let addr = Wallet::public_key_to_addr(&public_key);
        return Self {
            private_key,
            public_key,
            addr,
        }
    }

    pub fn sign_transaction(&self,transaction:&mut Transaction){
        let data=bincode::serialize(&transaction.info).unwrap();
        let hashed=digest::digest(&SHA256, &data);
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
        let signature=self.private_key.sign(padding, hashed.as_ref()).unwrap();
        transaction.sender_signature=signature;
    }

    pub fn create_signed_transaction(
        &self,sender_addr:String,receiver_addr:String,puzzle:Puzzle,value:i32
        )->Transaction{
        let mut transaction=Transaction::new(
            sender_addr, Wallet::public_key_to_string(&self.public_key), receiver_addr, puzzle, value);
        self.sign_transaction(&mut transaction);
        return transaction;
    }

    pub fn signature_to_string(signature:&Vec<u8>)->String{
        base64::encode(signature)
    }

    pub fn string_to_signature(signature_str:&String)->Vec<u8>{
        base64::decode(signature_str).unwrap()
    }

    fn public_key_to_addr(public_key:&RsaPublicKey) -> String {
        let bytes = public_key.n().to_bytes_be();
        let hash = Sha256::digest(&bytes);
        hex::encode(hash)
    }
    pub fn print(&self) {
        println!("Wallet:");
        println!("  Private Key: {:?}", Wallet::private_key_to_string(&self.private_key));
        println!("  Public Key: {:?}", Wallet::public_key_to_string(&self.public_key));
        println!("  Address: {}", self.addr);
    }

    pub fn to_transfer(&self)->WalletTransfer{
        return WalletTransfer{
            private_key:Wallet::private_key_to_string(&self.private_key),
            public_key:Wallet::public_key_to_string(&self.public_key),
            addr:self.addr.clone(),
        };
    }

    pub fn private_key_to_string(private_key:&RsaPrivateKey)->String{
        pkcs1::ToRsaPrivateKey::to_pkcs1_pem(private_key).unwrap().to_string()
    }

    pub fn public_key_to_string(public_key:&RsaPublicKey)->String{
        pkcs1::ToRsaPublicKey::to_pkcs1_pem(public_key).unwrap()
    }

    pub fn string_to_private_key(private_key_string: &str) -> RsaPrivateKey {
        RsaPrivateKey::from_pkcs1_pem(private_key_string).unwrap()
    }

    pub fn string_to_public_key(public_key_string: &str) -> RsaPublicKey {
        RsaPublicKey::from_pkcs1_pem(public_key_string).unwrap()
    }

}
