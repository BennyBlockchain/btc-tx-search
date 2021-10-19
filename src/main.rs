extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("btc-tx-search")
        .version("0.0.1")
        .author("Ben Schroth <ben@styng.social")
        .about("A Bitcoin CLI tool that can search transactions by address or txid.")
        .subcommand(
            SubCommand::with_name("account")
                .about("Searches UTXO set of a provided Account.")
                .arg(
                    Arg::with_name("pubkey")
                        .help("The account pubkey to search.")
                        .value_name("PUBKEY")
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

    if matches.is_present("account") {
        println!("Searching UTXO set of pubkey...");
    }

    if matches.is_present("tx") {
        println!("Searching UTXO info...")
    }


    if let Some(matches) = matches.subcommand_matches("account") {
        println!("UTXO set of PUBKEY: {}", matches.value_of("pubkey").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("tx") {
        println!("UTXO set of TXID: {}", matches.value_of("id").unwrap());
    }
    
}
