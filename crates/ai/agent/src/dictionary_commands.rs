//! The dictionary commands.

use std::path::Path;

use ai_tools::{check_entries, dictionary_guide, existing_entries, send_entries};
use dictionary::WordKey;

use crate::cli::DictionaryCommand;
use crate::data::{inbox, open_dictionary};
use crate::delivery_name::delivery_name;
use crate::outcome::{Failure, Outcome};
use crate::text_input::read_text;

pub async fn run(command: DictionaryCommand, data_dir: &Path) -> Result<Outcome, Failure> {
    match command {
        DictionaryCommand::Guide => {
            print!("{}", dictionary_guide());
        }
        DictionaryCommand::Export(args) => {
            let dictionary = open_dictionary(data_dir).await?;
            let exported = dictionary.export(&args.filter(), args.limit, false).await?;
            print!("{}", exported.text);
        }
        DictionaryCommand::Existing { file } => {
            let text = read_text(&file)?;
            let dictionary = open_dictionary(data_dir).await?;
            for found in existing_entries(&dictionary, &text).await? {
                println!("第 {} 行：{}", found.line, describe(&found.word));
            }
        }
        DictionaryCommand::Check { file } => {
            return Ok(Outcome::from_problems(check_entries(&read_text(&file)?)));
        }
        DictionaryCommand::Send { file } => {
            let name = delivery_name(&file)?;
            let text = read_text(&file)?;
            send_entries(&inbox(data_dir)?, &name, &text)?;
            println!("已送进 inbox：{name}");
        }
    }
    Ok(Outcome::Done)
}

fn describe(word: &WordKey) -> String {
    if word.origin.is_empty() {
        word.writing.clone()
    } else {
        format!("{}（词源：{}）", word.writing, word.origin)
    }
}
