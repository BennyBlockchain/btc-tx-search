// use reqwest;
use serde_derive::{Serialize, Deserialize};
use reqwest::{Error, Url};

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