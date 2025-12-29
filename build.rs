use std::fs;
use std::io::Write;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let ecu_mask_file = Path::new("ecu_mask.txt");
    let ecu_mask_content = fs::read_to_string(ecu_mask_file).expect("Failed to read ecu_mask.txt");

    // Generate the ECU mask function
    let mut code = String::from("pub fn get_matched_mask(ecu: &str, level: u8) -> Option<&'static str> {\n");
    code.push_str("    match (ecu, level) {\n");
    
    // Collect unique ECU types and security levels for enum generation
    let mut ecu_types = HashSet::new();
    let mut security_levels = HashSet::new();
    
    for line in ecu_mask_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 4 && parts[2] == "=" {
            let ecu = parts[0];
            let level = parts[1];
            let mask = parts[3];

            // Add the ECU type and security level to our sets of unique values
            ecu_types.insert(ecu);
            security_levels.insert(level);
            
            code.push_str(&format!(
                "        (\"{}\", {}) => Some(\"{}\"),\n", 
                ecu, level, mask
            ));
        }
    }

    code.push_str("        _ => None, \n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    
    // Generate the EcuType enum
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum EcuType {\n");
    
    // Sort the ECU types for consistent output
    let mut sorted_ecu_types: Vec<&str> = ecu_types.into_iter().collect();
    sorted_ecu_types.sort();
    
    for ecu in &sorted_ecu_types {
        // Convert the ECU name to proper Rust naming convention (upper camel case)
        let variant_name = if ecu.starts_with("x") {
            // Special case for xNav -> XNav
            ecu.replacen("x", "X", 1).replace("_", "")
        } else {
            ecu.replace("_", "")
        };
        code.push_str(&format!("    {},\n", variant_name));
    }
    
    code.push_str("}\n\n");
    
    // Generate the Display implementation for EcuType
    code.push_str("impl std::fmt::Display for EcuType {\n");
    code.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    code.push_str("        match self {\n");
    
    for ecu in &sorted_ecu_types {
        // Convert the ECU name to proper Rust naming convention (upper camel case)
        let variant_name = if ecu.starts_with("x") {
            // Special case for xNav -> XNav
            ecu.replacen("x", "X", 1).replace("_", "")
        } else {
            ecu.replace("_", "")
        };
        code.push_str(&format!(
            "            EcuType::{} => write!(f, \"{}\"),\n", 
            variant_name, ecu
        ));
    }
    
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    
    // Generate a function to convert string to EcuType
    code.push_str("pub fn str_to_ecu_type(s: &str) -> Option<EcuType> {\n");
    code.push_str("    match s {\n");
    
    for ecu in &sorted_ecu_types {
        // Convert the ECU name to proper Rust naming convention (upper camel case)
        let variant_name = if ecu.starts_with("x") {
            // Special case for xNav -> XNav
            ecu.replacen("x", "X", 1).replace("_", "")
        } else {
            ecu.replace("_", "")
        };
        code.push_str(&format!(
            "        \"{}\" => Some(EcuType::{}),\n", 
            ecu, variant_name
        ));
    }
    
    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    
    // Generate a function to get all EcuTypes
    code.push_str("pub fn all_ecu_types() -> Vec<EcuType> {\n");
    code.push_str("    vec![\n");
    
    for ecu in &sorted_ecu_types {
        // Convert the ECU name to proper Rust naming convention (upper camel case)
        let variant_name = if ecu.starts_with("x") {
            // Special case for xNav -> XNav
            ecu.replacen("x", "X", 1).replace("_", "")
        } else {
            ecu.replace("_", "")
        };
        code.push_str(&format!("        EcuType::{},\n", variant_name));
    }
    
    code.push_str("    ]\n");
    code.push_str("}\n");
    
    // Generate the SecurityLevel enum
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\n");
    code.push_str("pub enum SecurityLevel {\n");
    
    // Sort the security levels for consistent output
    let mut sorted_security_levels: Vec<&str> = security_levels.into_iter().collect();
    sorted_security_levels.sort();
    
    // Add default attribute to the first security level
    if !sorted_security_levels.is_empty() {
        code.push_str("    #[default]\n");
    }
    
    for level in &sorted_security_levels {
        code.push_str(&format!("    Level{},\n", level));
    }
    
    code.push_str("}\n\n");
    
    // Generate the Display implementation for SecurityLevel
    code.push_str("impl std::fmt::Display for SecurityLevel {\n");
    code.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    code.push_str("        match self {\n");
    
    for level in &sorted_security_levels {
        code.push_str(&format!(
            "            SecurityLevel::Level{} => write!(f, \"{}\"),\n", 
            level, level
        ));
    }
    
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    
    // Generate a function to get all SecurityLevels
    code.push_str("pub fn all_security_levels() -> Vec<SecurityLevel> {\n");
    code.push_str("    vec![\n");
    
    for level in &sorted_security_levels {
        code.push_str(&format!("        SecurityLevel::Level{},\n", level));
    }
    
    code.push_str("    ]\n");
    code.push_str("}\n");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("generated_ecu_mask.rs");
    let mut file = fs::File::create(&dest_path).expect("Failed to create generated_ecu_mask.rs");
    file.write_all(code.as_bytes()).expect("Failed to write generated_ecu_mask.rs");
}
