mod cli;
mod config;

use clap::Parser;
use cli::parser::Cli;
use cli::run::run;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli: Cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}
