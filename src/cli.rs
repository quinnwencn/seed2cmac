// src/cli.rs
use clap::{Arg, Command};
use std::error::Error;

#[derive(Debug)]
pub struct CliArg {
    pub seed: String,
    pub key: String,
    pub ecu: String,    
}

impl CliArg {
    pub fn new(seed: String, key: String, ecu: String) -> Self {
        Self { seed, key, ecu }
    }
}

pub fn parse_cli() -> Result<CliArg, Box<dyn Error>> {
    let matches = Command::new("Seed2Key")
        .version("0.1.0")
        .author("Quinn")
        .about("A tool to generate CMAC from seed.")
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .value_name("SEED")
                .help("Set the seed value")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .value_name("KEY")
                .help("Set the key value")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("ecu")
                .short('e')
                .long("ecu")
                .value_name("ECU")
                .help("Set the ECU value")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    if !matches.args_present() {
        println!("Usage: seed2key [OPTIONS]");
        println!("Options:");
        println!("  -s, --seed <SEED>    Set the seed value");
        println!("  -k, --key <KEY>      Set the key value");
        println!("  -e, --ecu <ECU>      Set the ECU name");
        println!("  -h, --help           Print help information");
        return Err("Invalid input".into());
    }

    let seed = matches.get_one::<String>("seed").ok_or("seed not found")?.to_owned();
    let key = matches.get_one::<String>("key").ok_or("key not found")?.to_owned();
    let ecu = matches.get_one::<String>("ecu").ok_or("ecu not found")?.to_owned();

    Ok(CliArg::new(seed, key, ecu))
}
