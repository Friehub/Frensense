// [frensense]
// observation: clap reads a secret argument from an environment variable using #[arg(env = "SECRET")] but the process startup logs all environment variables (e.g., via env_logger or systemd journal), exposing the secret.
// impact: If the process or system logs environment variables at startup (common in CI, systemd, or container runtimes), the secret value is written to logs accessible to operators and log aggregation systems.
// improvement: Read secrets from files or dedicated secret stores instead of environment variables, or ensure env vars are filtered from logs.

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "DATABASE_PASSWORD")]
    db_password: String,
}

fn main() {
    let _args = Args::parse();
}

#[derive(Parser)]
struct ApiArgs {
    #[arg(long, env = "API_KEY")]
    api_key: String,
}

fn run() {
    let _args = ApiArgs::parse();
}
