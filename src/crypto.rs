use aes::Aes128;
use cmac::{Cmac, Mac}; 
use std::error::Error;

// Import from parent crate
use crate::util::{hex_string_to_bytes, bytes_to_hex_string};

/// Calculates CMAC using AES-128
/// 
/// # Arguments
/// * `key` - The key bytes for CMAC calculation
/// * `data` - The data to calculate CMAC for
/// 
/// # Returns
/// * `Result<Vec<u8>, Box<dyn Error>>` - The calculated CMAC bytes or an error
pub fn calculate_cmac(key: &[u8], data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut mac = Cmac::<Aes128>::new_from_slice(key)
        .map_err(|e| format!("Invalid key length: {:?}", e))?;

    mac.update(data);
    let result = mac.finalize().into_bytes();
    
    Ok(result.to_vec())
}

/// Calculates the CMAC key based on key and mask_value
/// 
/// # Arguments
/// * `key_input` - Hex string of the key
/// * `mask_value` - Pre-calculated mask value (after XOR operation)
/// 
/// # Returns
/// * `Result<String, Box<dyn Error>>` - The calculated CMAC key as a hex string or an error
pub fn calculate_cmac_key(
    key_input: &str,
    mask_value: &[u8]
) -> Result<String, Box<dyn Error>> {
    // Validate and convert key to bytes
    if key_input.is_empty() {
        return Err("输入的Key不能为空".into());
    }
    
    let key = match hex_string_to_bytes(key_input) {
        Ok(bytes) => bytes,
        Err(_) => return Err("无效的Key输入：必须是32个字符的十六进制字符串".into()),
    };
    
    // Calculate CMAC
    let cmac = match calculate_cmac(&key, mask_value) {
        Ok(bytes) => bytes,
        Err(e) => return Err(format!("CMAC计算失败: {}", e).into()),
    };
    
    // Convert result to hex string
    Ok(bytes_to_hex_string(&cmac))
}
