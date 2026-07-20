// SAFE: Secret arguments are read from environment variables only, not exposed as CLI flags in help.

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "APP_PASSWORD")]
    password: String,
}

fn main() {
    let _args = Args::parse();
}
