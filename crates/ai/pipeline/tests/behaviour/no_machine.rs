//! Jobs that fail before any model runs; no GPU machine needed.

use ai_invoker::{InvokeError, Invoker, InvokerSettings};
use ai_pipeline::{Language, PipelineError, design_voice, speak_lines};
use futures_util::StreamExt;

use crate::support::{item, sample};

fn no_machine(model: &str) -> PipelineError {
    PipelineError::Invoke(InvokeError::NoMachine {
        model: model.into(),
        tried: vec![],
    })
}

#[tokio::test]
async fn an_item_without_its_sample_fails_before_any_model_runs() {
    let invoker = Invoker::new(InvokerSettings::default());
    let items = vec![item("美咲", Language::Zh, "早上好。")];
    let samples = vec![sample("美咲", Language::Ja, vec![1], "おはよう。")];
    let spoken: Vec<_> = speak_lines(&invoker, items, samples).collect().await;
    assert_eq!(
        spoken,
        vec![Err(PipelineError::NoSample {
            voice: "美咲".into(),
            language: Language::Zh,
        })]
    );
}

#[tokio::test]
async fn the_first_error_ends_the_stream() {
    let invoker = Invoker::new(InvokerSettings::default());
    let items = vec![
        item("美咲", Language::Ja, "おはよう。"),
        item("美咲", Language::Ja, "またね。"),
    ];
    let samples = vec![sample("美咲", Language::Ja, vec![1], "はじめまして。")];
    let spoken: Vec<_> = speak_lines(&invoker, items, samples).collect().await;
    assert_eq!(spoken, vec![Err(no_machine("qwen3-tts-1.7b-base"))]);
}

#[tokio::test]
async fn no_items_need_no_machine() {
    let invoker = Invoker::new(InvokerSettings::default());
    let spoken: Vec<_> = speak_lines(&invoker, vec![], vec![]).collect().await;
    assert!(spoken.is_empty());
}

#[tokio::test]
async fn designing_a_voice_needs_the_voice_design_model() {
    let invoker = Invoker::new(InvokerSettings::default());
    let designed = design_voice(&invoker, "明亮的声音。", Language::Ja, "おはよう。", 7).await;
    assert_eq!(designed, Err(no_machine("qwen3-tts-1.7b-voicedesign")));
}
