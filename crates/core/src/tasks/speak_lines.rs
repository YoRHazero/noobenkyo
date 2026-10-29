//! Speaking the lines an episode lacks audio for: listening lists the items
//! and their references, ai speaks them one by one, and each is stored as it comes,
//! so a cancelled or failed run keeps what it made and the next one picks
//! up the rest.

use std::collections::BTreeSet;
use std::pin::pin;

use futures_util::StreamExt;
use listening::{Language, ListeningService, SpeechItem, Uuid};
use tauri::{AppHandle, Manager, Runtime};

use crate::app_state::AppState;
use crate::command_error::CommandError;
use crate::convert::{speak_item, voice_sample};
use crate::tasks::Progress;

pub async fn speak_lines<R: Runtime>(
    app: AppHandle<R>,
    episode: Uuid,
    progress: Progress<R>,
) -> Result<(), CommandError> {
    let state = app.state::<AppState>();
    let items = state.listening.missing_audio(episode).await?;
    let samples = samples(&state.listening, &items).await?;
    let total = items.len() as u32;
    progress.report(0, total);

    let speak_items = items.iter().map(speak_item::to_ai).collect();
    let mut spoken = pin!(state.ai.speak_lines(speak_items, samples));
    let mut done = 0;
    while let Some(result) = spoken.next().await {
        let spoken = result?;
        let item = &items[spoken.index];
        if let Err(error) = state
            .listening
            .save_audio(item.line, item.language, &spoken.wav)
            .await
        {
            // The line was deleted while it was spoken: skip it.
            log::warn!("line {} kept no audio: {error}", item.line.0);
        }
        done += 1;
        progress.report(done, total);
    }
    Ok(())
}

/// One reference per voice and language the items use, fetched once each.
/// A voice that has lost its reference meanwhile gets none, and ai says so
/// when its first item comes up.
async fn samples(
    listening: &ListeningService,
    items: &[SpeechItem],
) -> Result<Vec<ai::VoiceSample>, CommandError> {
    let needed: BTreeSet<(&str, Language)> = items
        .iter()
        .map(|item| (item.voice.as_str(), item.language))
        .collect();
    let mut samples = Vec::new();
    for (voice, language) in needed {
        if let Some(reference) = listening.reference(voice, language).await? {
            samples.push(voice_sample::to_ai(voice, language, reference));
        }
    }
    Ok(samples)
}
