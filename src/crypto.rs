use aes::Aes128;
use cmac::{Cmac, Mac}; 
use std::error::Error;

pub fn calculate_cmac(key: &[u8], data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut mac = Cmac::<Aes128>::new_from_slice(key).map_err(|e| format!("Invalid key length: {:?}", e))?;

    mac.update(data);
    let result = mac.finalize().into_bytes();
    
    Ok(result.to_vec())
}
