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

#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn test_txids() {
        let txid = String::from("a35a6e97be19c719fd7eb5d7367d248fc1398c7a3b3739be612b3693ed3d1af6");
        let test_tx = Transaction::get(txid).await.unwrap();

        assert_eq!(test_tx.txid, "a35a6e97be19c719fd7eb5d7367d248fc1398c7a3b3739be612b3693ed3d1af6");
        assert_eq!(test_tx.vin[0].txid, "15dcd293b6c2ce093d3e0ccb735aa96d7f1852bade9f6cef1ca27912f64bc2cc");
    }

    #[tokio::test]
    async fn test_address_balance() {
        let address = String::from("1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY");
        let test_address = Address::get(address).await.unwrap();

        assert_eq!(test_address.address, "1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY");
        assert_eq!(test_address.chain_stats.spent_txo_sum, 521251113989925);
    }
}
