//! The listening commands. Data comes out as JSON, which an agent reads
//! without guessing.

use std::path::Path;

use ai_tools::{ScriptTarget, SendError, listening_guide, send_script};
use listening::check_script;
use serde::Serialize;

use crate::cli::ListeningCommand;
use crate::data::{inbox, open_listening};
use crate::outcome::{Failure, Outcome};
use crate::text_input::read_text;

pub async fn run(command: ListeningCommand, data_dir: &Path) -> Result<Outcome, Failure> {
    match command {
        ListeningCommand::Guide => {
            print!("{}", listening_guide());
        }
        ListeningCommand::Worlds => {
            print_json(&open_listening(data_dir).await?.worlds().await?);
        }
        ListeningCommand::Episodes { world } => {
            print_json(&open_listening(data_dir).await?.episodes(&world).await?);
        }
        ListeningCommand::Context {
            world,
            predecessors,
            append,
        } => {
            let listening = open_listening(data_dir).await?;
            print_json(
                &listening
                    .script_context(&world, &predecessors, append)
                    .await?,
            );
        }
        ListeningCommand::Check { file } => {
            return Ok(Outcome::from_problems(check_script(&read_text(&file)?)));
        }
        ListeningCommand::Send {
            file,
            world,
            append,
        } => {
            let target = match (world, append) {
                (_, Some(episode)) => ScriptTarget::Append { episode },
                (Some(world), None) => ScriptTarget::NewEpisode { world },
                (None, None) => unreachable!("clap requires --world or --append"),
            };
            let text = read_text(&file)?;
            let listening = open_listening(data_dir).await?;
            match send_script(&listening, &inbox(data_dir)?, &text, target).await {
                Ok(()) => println!("已送进 inbox"),
                Err(SendError::Problems(problems)) => return Ok(Outcome::Problems(problems)),
                Err(error) => return Err(error.into()),
            }
        }
    }
    Ok(Outcome::Done)
}

fn print_json(value: &impl Serialize) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("listening values always serialize")
    );
}
