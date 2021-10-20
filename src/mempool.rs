// use reqwest;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Transaction {
    txid: String,
    version: u8,
    locktime: u8,
    vin: Vec<Vin>,
    vout: Vec<Prevout>,
    size: u32,
    weight: u64,
    status: Status
}

#[derive(Serialize, Deserialize)]
struct Vin {
    txid: String,
    vout: u8,
    prevout: Prevout,
    scriptsig: String,
    scriptsig_asm: String,
    witness: Vec<String>,
    is_coinbase: bool,
    sequence: u32,
    inner_redeemscript_asm: String
}

#[derive(Serialize, Deserialize)]
struct Prevout {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: u32
}

#[derive(Serialize, Deserialize)]
struct Status {
    confirmed: bool,
    block_height: u32,
    block_hash: String,
    block_time: u32
}

// pub async fn get_tx(id: String) -> Option<T, E> {
//     let res = reqwest::get("https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521")
//         .await?
//         .text()
//         .await?;
//     println!("This bitch, worked");

// }

pub fn test() {
    println!("This bitch, worked"); 
}


