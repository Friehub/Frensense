use std::env;
use std::process::Command;

fn main() {
    eprintln!("\n=======================================================");
    eprintln!("WARNING: The 'genesense' package has been renamed to 'frensense'.");
    eprintln!("Please update your scripts and run: cargo install frensense");
    eprintln!("=======================================================\n");

    let args: Vec<String> = env::args().skip(1).collect();

    let status = Command::new("frensense")
        .args(&args)
        .status()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute 'frensense'. Please ensure it is installed in your PATH.");
            eprintln!("You can install it by running: cargo install frensense");
            std::process::exit(1);
        });

    std::process::exit(status.code().unwrap_or(1));
}
