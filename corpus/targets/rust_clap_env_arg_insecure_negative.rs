// SAFE: The secret is read from a file path specified via argument, never from an environment variable.

use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    db_password_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let _password = fs::read_to_string(&args.db_password_file)?;
    Ok(())
}
