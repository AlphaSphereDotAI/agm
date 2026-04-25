mod cli;
mod config;

use clap::Parser;
use cli::parser::Cli;
use cli::run::run;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli: Cli = Cli::parse();
    let version = env!("CARGO_PKG_VERSION");

    match run(cli, version) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}
