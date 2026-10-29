//! Designing a voice and speaking in it on a real GPU machine.

use ai_pipeline::{Language, design_voice, speak_lines};
use futures_util::StreamExt;

use crate::support::{gpu_invoker, is_wav, item, sample};

const PROMPT: &str = "20岁的女性，声音明亮，语速适中。";
const SAMPLE_TEXT: &str = "はじめまして。今日はいい天気ですね。一緒に散歩しませんか。";

#[tokio::test]
#[ignore = "needs a GPU machine"]
async fn designs_a_voice_and_speaks_in_it() {
    let invoker = gpu_invoker();
    let reference = design_voice(&invoker, PROMPT, Language::Ja, SAMPLE_TEXT, 7)
        .await
        .unwrap();
    assert!(is_wav(&reference));

    let items = vec![
        item("美咲", Language::Ja, "おはよう。"),
        item("美咲", Language::Ja, "またね。"),
    ];
    let samples = vec![sample("美咲", Language::Ja, reference, SAMPLE_TEXT)];
    let spoken: Vec<_> = speak_lines(&invoker, items, samples)
        .map(Result::unwrap)
        .collect()
        .await;
    assert_eq!(
        spoken.iter().map(|spoken| spoken.index).collect::<Vec<_>>(),
        [0, 1]
    );
    assert!(spoken.iter().all(|spoken| is_wav(&spoken.wav)));
}

#[tokio::test]
#[ignore = "needs a GPU machine"]
async fn dropping_the_stream_stops_after_the_item_under_way() {
    let invoker = gpu_invoker();
    let reference = design_voice(&invoker, PROMPT, Language::Ja, SAMPLE_TEXT, 7)
        .await
        .unwrap();
    let items = vec![
        item("美咲", Language::Ja, "おはよう。"),
        item("美咲", Language::Ja, "またね。"),
        item("美咲", Language::Ja, "おやすみ。"),
    ];
    let samples = vec![sample("美咲", Language::Ja, reference, SAMPLE_TEXT)];
    let first: Vec<_> = speak_lines(&invoker, items, samples)
        .take(1)
        .collect()
        .await;
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].as_ref().unwrap().index, 0);
}
