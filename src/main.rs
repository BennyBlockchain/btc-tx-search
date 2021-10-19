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
                        .index(1)
                        .required(true)
                )
        )
        .get_matches();

    if matches.is_present("account") {
        println!("Searching UTXO set of pubkey...");
    }

    if let Some(matches) = matches.subcommand_matches("account") {
        println!("UTXO set of: {}", matches.value_of("pubkey").unwrap());
    }
    
}
