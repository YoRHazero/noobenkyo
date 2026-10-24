//! Demo data for trying the listening pages: a world with a cast and two
//! episodes, voiced by qwen-tts on the GPU machine. Driven by
//! scripts/demo-data/demo-data.sh; not part of the app.
//!
//!     demo_data seed <data dir> <cast.json> <references dir>
//!         Creates the characters from their references, the world and its
//!         cast, stores the episodes, and prints what audio they lack as JSON.
//!         Does nothing when the world is already there.
//!     demo_data save <data dir> <jobs.json> <audio dir>
//!         Stores the audio made for each job.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use listening::{Gender, Language, LineId, ListeningService, NewCharacter, NewReference, Role};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Cast {
    world: String,
    setting: String,
    characters: Vec<Character>,
    /// Script files, relative to cast.json.
    episodes: Vec<PathBuf>,
}

#[derive(Deserialize)]
struct Character {
    name: String,
    kana: String,
    gender: Gender,
    age: u32,
    role: Role,
    prompt: String,
    /// What is said in each reference.
    references: BTreeMap<Language, String>,
}

/// A recording to make: the shape `missing_audio` gives, as JSON.
#[derive(Serialize, Deserialize)]
struct Job {
    line: LineId,
    language: Language,
    text: String,
    voice: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["seed", data, cast, refs] => seed(Path::new(data), Path::new(cast), Path::new(refs)).await,
        ["save", data, jobs, audio] => {
            save(Path::new(data), Path::new(jobs), Path::new(audio)).await
        }
        _ => panic!(
            "usage: demo_data seed <data> <cast.json> <refs> | save <data> <jobs.json> <audio>"
        ),
    }
}

async fn open(data: &Path) -> ListeningService {
    ListeningService::open(&data.join("listening"))
        .await
        .expect("cannot open listening")
}

fn language_code(language: Language) -> &'static str {
    match language {
        Language::Ja => "ja",
        Language::Zh => "zh",
    }
}

async fn seed(data: &Path, cast_path: &Path, refs: &Path) {
    let service = open(data).await;
    let cast: Cast =
        serde_json::from_slice(&std::fs::read(cast_path).expect("cannot read cast.json"))
            .expect("bad cast.json");
    let worlds = service.worlds().await.expect("cannot list worlds");
    if worlds.iter().any(|world| world.name == cast.world) {
        eprintln!("世界 {} 已经存在，什么都不做", cast.world);
        println!("[]");
        return;
    }

    for character in &cast.characters {
        let references = character
            .references
            .iter()
            .map(|(language, text)| NewReference {
                language: *language,
                audio: std::fs::read(refs.join(format!(
                    "{}-{}.wav",
                    character.name,
                    language_code(*language)
                )))
                .expect("missing reference"),
                text: text.clone(),
            })
            .collect();
        let problems = service
            .create_character(&NewCharacter {
                name: character.name.clone(),
                kana: character.kana.clone(),
                gender: character.gender,
                age: character.age,
                prompt: Some(character.prompt.clone()),
                references,
            })
            .await
            .expect("cannot create character");
        assert!(problems.is_empty(), "{}: {problems:?}", character.name);
    }
    service
        .create_world(&cast.world, &cast.setting)
        .await
        .expect("cannot create world");
    for character in &cast.characters {
        service
            .add_cast(&cast.world, &character.name, &character.role)
            .await
            .expect("cannot cast");
    }

    let base = cast_path.parent().unwrap_or(Path::new("."));
    let mut jobs = Vec::new();
    for script in &cast.episodes {
        let text = std::fs::read_to_string(base.join(script)).expect("cannot read script");
        let episode = service
            .import_script(&cast.world, &text)
            .await
            .expect("cannot store script");
        for item in service
            .missing_audio(episode)
            .await
            .expect("cannot list audio")
        {
            jobs.push(Job {
                line: item.line,
                language: item.language,
                text: item.text,
                voice: item.voice,
            });
        }
    }
    println!("{}", serde_json::to_string_pretty(&jobs).expect("jobs"));
}

async fn save(data: &Path, jobs_path: &Path, audio: &Path) {
    let service = open(data).await;
    let jobs: Vec<Job> =
        serde_json::from_slice(&std::fs::read(jobs_path).expect("cannot read jobs.json"))
            .expect("bad jobs.json");
    for job in &jobs {
        let file = audio.join(format!(
            "{}-{}.wav",
            job.line.0,
            language_code(job.language)
        ));
        let wav = std::fs::read(&file).expect("missing audio");
        service
            .save_audio(job.line, job.language, &wav)
            .await
            .expect("cannot save audio");
    }
    eprintln!("已保存：{} 段", jobs.len());
}
