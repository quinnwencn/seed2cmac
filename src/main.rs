mod cli;

use std::result::Result;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli_arg = cli::parse_cli()?;
    println!("{:?}", cli_arg);

    Ok(())
}
