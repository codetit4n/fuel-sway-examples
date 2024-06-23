extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::str::FromStr;

use fuels::{crypto::SecretKey, prelude::*};

/// Deploy the contract to testnet
#[tokio::main]
async fn main() {
    dotenv().ok();

    let pvt_key = &env::var("PVT_KEY").expect("Failed to get PVT_KEY from .env");

    let provider = Provider::connect("testnet.fuel.network")
        .await
        .expect("Failed to connect to testnet");

    let secret_key = SecretKey::from_str(pvt_key).expect("Failed to parse secret key");

    let wallet = WalletUnlocked::new_from_private_key(secret_key, Some(provider));
    dbg!(wallet);
}
