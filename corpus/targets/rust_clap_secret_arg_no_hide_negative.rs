// SAFE: Secret arguments are marked with hide_from_help(true) so they are not displayed in help output.

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, short, hide_from_help = true)]
    password: String,
}

fn main() {
    let _args = Args::parse();
}
