//! `noobenkyo`: the command line an outside agent (Claude Code, Codex, …)
//! uses to read the data, check what it wrote, and send it to the app. It
//! reads the data directly, so the app need not be open, and it writes
//! nothing but the inbox.
//!
//! Exit codes: 0 done; 1 the text has problems, printed one per line;
//! 2 the command could not run, the reason on standard error.

mod cli;
mod data;
mod delivery_name;
mod dictionary_commands;
mod listening_commands;
mod outcome;
mod text_input;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Group};
use outcome::{Failure, Outcome};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("a single-threaded runtime always builds");
    match runtime.block_on(run(cli)) {
        Ok(Outcome::Done) => ExitCode::SUCCESS,
        Ok(Outcome::Problems(problems)) => {
            for problem in problems {
                println!("{problem}");
            }
            ExitCode::from(1)
        }
        Err(Failure(reason)) => {
            eprintln!("{reason}");
            ExitCode::from(2)
        }
    }
}

async fn run(cli: Cli) -> Result<Outcome, Failure> {
    let data_dir = data::data_dir()?;
    match cli.group {
        Group::Dictionary(command) => dictionary_commands::run(command, &data_dir).await,
        Group::Listening(command) => listening_commands::run(command, &data_dir).await,
    }
}
