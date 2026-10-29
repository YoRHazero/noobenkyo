use ai_invoker::{Invoker, TtsDesignRequest};

use crate::{Language, PipelineError};

/// Makes up a voice from `prompt` with `model`, a TtsDesign model, and has
/// it say `text`, to become a reference recording. Returns the WAV. With
/// the same prompt and seed, the two languages of a character come out as
/// the same person.
pub async fn design_voice(
    invoker: &Invoker,
    model: &str,
    prompt: &str,
    language: Language,
    text: &str,
    seed: u64,
) -> Result<Vec<u8>, PipelineError> {
    let request = TtsDesignRequest {
        text: text.to_owned(),
        language: language.tts(),
        instruct: prompt.to_owned(),
        seed: Some(seed),
    };
    let reply = invoker.tts_design(model, request).await?;
    Ok(reply.wav)
}
