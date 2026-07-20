// [frensense]
// observation: A clap CLI argument for a secret (password, token, API key) is not marked as hide_from_help(true), causing the secret value or its description to appear in --help output.
// impact: Secrets or their names/descriptions are visible in help text, which may be displayed on shared terminals, CI logs, or documented in screenshots, leading to credential leakage.
// improvement: Mark secret arguments with .hide_from_help(true) so they do not appear in help output.

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    password: String,
}

fn main() {
    let _args = Args::parse();
}

#[derive(Parser)]
struct DeployArgs {
    #[arg(long)]
    api_token: String,
}

fn deploy() {
    let _args = DeployArgs::parse();
}
