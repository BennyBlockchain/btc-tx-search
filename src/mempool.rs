// use reqwest;
use serde_derive::{Serialize, Deserialize};
use reqwest::{Error, Url};
use std::fmt;
use colored::*;

/**
 * * Transaction Structs
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    txid: String,
    version: u8,
    locktime: u8,
    vin: Vec<Vin>,
    vout: Vec<Prevout>,
    size: u32,
    weight: u64,
    status: Status
}

#[derive(Serialize, Deserialize, Debug)]
struct Vin {
    txid: String,
    vout: u8,
    prevout: Prevout,
    scriptsig: String,
    scriptsig_asm: String,
    witness: Option<Vec<String>>,
    is_coinbase: bool,
    sequence: u32,
    inner_redeemscript_asm: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Prevout {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    confirmed: bool,
    block_height: u32,
    block_hash: String,
    block_time: u32
}

impl Transaction {
    pub async fn get(id: String) -> Result<Self, Error> {
        let url: String = format!("https://mempool.space/api/tx/{}", id);
        let url: Url = Url::parse(&url).unwrap();

        let resp = reqwest::get(url)
            .await?
            .json::<Transaction>()
            .await?;
        
        Ok(resp)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.txid.green())
    }
}

/**
 * * Address Structs
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    address: String,
    chain_stats: Stats,
    mempool_stats: Stats    
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    funded_txo_count: u32,
    funded_txo_sum: u64,
    spent_txo_count: u32,
    spent_txo_sum: u64,
    tx_count: u32
}

impl Address {
    pub async fn get(address: String) -> Result<Self, Error> {
        let url: String = format!("https://mempool.space/api/address/{}", address);
        let url: Url = Url::parse(&url).unwrap();

        let resp = reqwest::get(url)
            .await?
            .json::<Address>()
            .await?;

        Ok(resp)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address.red())
    }
}