use std::error::Error;
use hex;

use crate::cli;
use crate::crypto;

pub fn hex_string_to_bytes(hex_str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if hex_str.len() % 2 != 0 {
        return Err("Invalid hex string".into());
    }

    if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid hex string".into());
    }

    let mut bytes = vec![0; hex_str.len() / 2];
    hex::decode_to_slice(hex_str, &mut bytes)?;

    Ok(bytes)
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn process(arg: &cli::CliArg) -> Result<Vec<u8>, Box<dyn Error>> {
    let seed = hex_string_to_bytes(&arg.seed)?;
    let key = hex_string_to_bytes(&arg.key)?;
    let cmac = crypto::calculate_cmac(&key, &seed)?;
    Ok(cmac)
}
