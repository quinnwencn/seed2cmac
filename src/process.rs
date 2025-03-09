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


// high 8 bytes, low 8 bytes
fn ecu_mask(ecu: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    match ecu {
        "ECU1" => Ok(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00]),
        "ECU2" => Ok(vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]),
        "ECU3" => Ok(vec![0xff, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
        _ => Err("Invalid ECU".into()),
    }
}

fn xor_bytes(a: &Vec<u8>, b: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    if a.len() != b.len() {
        let s: String = format!("Length mismatch, a: {}, b: {}", a.len(), b.len());
        return Err(s.into());
    }

    Ok(a.iter()
     .zip(b.iter())
     .map(|(a, b)| a ^ b)
     .collect())
}

pub fn process(arg: &cli::CliArg) -> Result<Vec<u8>, Box<dyn Error>> {
    let seed = hex_string_to_bytes(&arg.seed)?;
    let key = hex_string_to_bytes(&arg.key)?;
    let mask = ecu_mask(&arg.ecu)?;

    let mask_value = xor_bytes(&seed, &mask)?;
    let cmac = crypto::calculate_cmac(&mask_value, &key)?;
    Ok(cmac)
}
