use std::result::Result;
use std::error::Error;

mod cli;
mod process;
mod crypto;

fn main() -> Result<(), Box<dyn Error>> {
    let cli_arg = cli::parse_cli()?;
    let cmac = process::process(&cli_arg)?;
    println!("CMAC: {}", process::bytes_to_hex_string(&cmac));

    Ok(())
}
