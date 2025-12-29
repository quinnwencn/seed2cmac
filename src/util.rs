use hex;
use std::error::Error;

pub fn hex_string_to_bytes(hex_str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if hex_str.len() != 32 {
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

pub fn xor_bytes(a: &Vec<u8>, b: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    if a.len() != b.len() {
        let s: String = format!("Length mismatch, a: {}, b: {}", a.len(), b.len());
        return Err(s.into());
    }

    Ok(a.iter()
     .zip(b.iter())
     .map(|(a, b)| a ^ b)
     .collect())
}