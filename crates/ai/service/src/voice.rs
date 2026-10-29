//! Speaking lines and designing voices, handed on to ai-pipeline under the
//! invoker's read lock.

use std::pin::pin;

use futures_util::{Stream, StreamExt};

use crate::{AiError, AiService, Language, SpeakItem, Spoken, VoiceSample};

impl AiService {
    /// Speaks the items in order, yielding each as soon as it is done. The
    /// first error is yielded and ends the stream; dropping the stream
    /// cancels it. Each item's voice and language need a sample.
    pub fn speak_lines(
        &self,
        items: Vec<SpeakItem>,
        samples: Vec<VoiceSample>,
    ) -> impl Stream<Item = Result<Spoken, AiError>> + Send + '_ {
        async_stream::stream! {
            if let Err(error) = self.ready() {
                yield Err(error);
                return;
            }
            let invoker = self.invoker.read().await;
            let mut spoken = pin!(ai_pipeline::speak_lines(&invoker, items, samples));
            while let Some(result) = spoken.next().await {
                yield result.map_err(AiError::from);
            }
        }
    }

    /// Makes up a voice from `prompt` saying `text`, to become a reference
    /// recording; returns the WAV. One seed for both languages of a
    /// character makes them sound like one person.
    pub async fn design_voice(
        &self,
        prompt: &str,
        language: Language,
        text: &str,
        seed: u64,
    ) -> Result<Vec<u8>, AiError> {
        self.ready()?;
        let invoker = self.invoker.read().await;
        Ok(ai_pipeline::design_voice(&invoker, prompt, language, text, seed).await?)
    }
}
