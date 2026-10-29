use ai_invoker::{Invoker, TtsCloneRequest};
use futures_util::Stream;

use crate::voice_sample::{self, VoiceSample};
use crate::{Language, PipelineError};

/// One recording to make. It carries no line id: what it belongs to is the
/// caller's business, and `Spoken::index` points back at it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpeakItem {
    /// The text to say, as the model reads it.
    pub text: String,
    pub language: Language,
    /// The voice whose sample in `language` to clone.
    pub voice: String,
}

/// One item done.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spoken {
    /// Its place among the items passed.
    pub index: usize,
    /// Mono 16-bit WAV, as the model wrote it.
    pub wav: Vec<u8>,
}

/// Speaks the items in order with `model`, a TtsClone model, and yields
/// each one as soon as it is done, so the caller can store it at once. The
/// first error is yielded and ends the stream. Dropping the stream stops it
/// and cancels the item under way.
pub fn speak_lines<'a>(
    invoker: &'a Invoker,
    model: &'a str,
    items: Vec<SpeakItem>,
    samples: Vec<VoiceSample>,
) -> impl Stream<Item = Result<Spoken, PipelineError>> + Send + 'a {
    async_stream::stream! {
        for (index, item) in items.into_iter().enumerate() {
            let spoken = speak(invoker, model, item, &samples)
                .await
                .map(|wav| Spoken { index, wav });
            let failed = spoken.is_err();
            yield spoken;
            if failed {
                break;
            }
        }
    }
}

async fn speak(
    invoker: &Invoker,
    model: &str,
    item: SpeakItem,
    samples: &[VoiceSample],
) -> Result<Vec<u8>, PipelineError> {
    let sample = voice_sample::find(samples, &item.voice, item.language).ok_or_else(|| {
        PipelineError::NoSample {
            voice: item.voice.clone(),
            language: item.language,
        }
    })?;
    let request = TtsCloneRequest {
        text: item.text,
        language: item.language.tts(),
        ref_audio: sample.audio.clone(),
        ref_text: sample.text.clone(),
    };
    let reply = invoker.tts_clone(model, request).await?;
    Ok(reply.wav)
}
