use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct Wallet{
    private_key:RsaPrivateKey,
    public_key:RsaPublicKey,
    addr:String,
}