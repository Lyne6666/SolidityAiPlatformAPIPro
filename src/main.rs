// src/main.rs
/*
 * Main executable for SolidityAiPlatformAPIPro
 */

use clap::Parser;
use solidityaiplatformapipro::{Result, run};

#[derive(Parser)]
#[command(version, about = "SolidityAiPlatformAPIPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
