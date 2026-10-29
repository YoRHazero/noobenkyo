//! Speaking lines without a machine to run them.

use ai::{AiService, Language, SpeakItem, VoiceSample};
use futures_util::StreamExt;

#[tokio::test]
async fn without_machines_the_first_item_fails_and_ends_it() {
    let dir = tempfile::tempdir().unwrap();
    let service = AiService::open(&dir.path().join("ai")).unwrap();
    let item = |text: &str| SpeakItem {
        text: text.into(),
        language: Language::Ja,
        voice: "美咲".into(),
    };
    let samples = vec![VoiceSample {
        voice: "美咲".into(),
        language: Language::Ja,
        audio: vec![1],
        text: "はじめまして。".into(),
    }];
    let spoken: Vec<_> = service
        .speak_lines(vec![item("おはよう。"), item("またね。")], samples)
        .collect()
        .await;
    assert_eq!(spoken.len(), 1);
    assert_eq!(
        spoken[0].as_ref().unwrap_err().to_string(),
        "没有能运行 qwen3-tts-1.7b-base 的机器"
    );
}
