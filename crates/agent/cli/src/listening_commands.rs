//! The listening commands. Data comes out as JSON, which an agent reads
//! without guessing.

use std::path::Path;

use agent_tools::{SendError, listening_guide, replace_script, send_script};
use inbox::{InboxSource, ScriptTarget};
use listening::check_script;
use serde::Serialize;

use crate::cli::ListeningCommand;
use crate::data::{inbox, open_listening};
use crate::delivery_name::delivery_name;
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
            replace,
        } => {
            let target = match (world, append) {
                (_, Some(episode)) => ScriptTarget::Append { episode },
                (Some(world), None) => ScriptTarget::NewEpisode { world },
                (None, None) => unreachable!("clap requires --world or --append"),
            };
            let name = delivery_name(&file)?;
            let text = read_text(&file)?;
            let listening = open_listening(data_dir).await?;
            let inbox = inbox(data_dir)?;
            let sent = if replace {
                replace_script(&listening, &inbox, &name, &text, target).await
            } else {
                send_script(&listening, &inbox, &name, &text, target).await
            };
            match sent {
                Ok(()) if replace => println!("已覆写 inbox 里的 {name}"),
                Ok(()) => println!("已送进 inbox：{name}"),
                Err(SendError::Problems(problems)) => return Ok(Outcome::Problems(problems)),
                Err(error) => return Err(error.into()),
            }
        }
        ListeningCommand::Show { name } => {
            let script = inbox(data_dir)?.require_script(InboxSource::Agent, &name)?;
            println!("{}", script.text.trim_end());
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
