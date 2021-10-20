use clap::{App, Arg, SubCommand};
mod mempool;
use mempool::Transaction;

#[tokio::main]
async fn main() {
    let matches = App::new("btc-tx-search")
        .version("v0.0.1")
        .author("Ben Schroth <ben@styng.social>")
        .about("A Bitcoin CLI tool that can search transactions by address or txid.")
        .subcommand(
            SubCommand::with_name("address")
                .about("Searches UTXO set w/ provided address.")
                .arg(
                    Arg::with_name("pubkey")
                        .help("The address to search.")
                        .value_name("ADDRESS")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("tx")
                .about("Search UTXO w/ provided txid.")
                .arg(
                    Arg::with_name("id")
                        .help("The transaction id for UTXO information.")
                        .value_name("TXID")
                        .index(1)
                        .required(true)
                )
        )
        .get_matches();

    if matches.is_present("address") {
        println!("Searching UTXO set of address...");
    }

    if matches.is_present("tx") {
        println!("Searching UTXO info...");
    }


    if let Some(matches) = matches.subcommand_matches("address") {
        println!("UTXO set of ADDRESS: {}", matches.value_of("pubkey").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("tx") {
        println!("UTXO set of TXID: {}", matches.value_of("id").unwrap());
        let id = matches.value_of("id").unwrap().to_string();
        let tx_call = Transaction::get(id).await;

        println!("Transaction:\n{:?}", tx_call);
    }
    
}
