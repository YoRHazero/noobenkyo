//! Speaking lines without a model picked, or without a machine to run it.

use ai::{AiService, AiSettings, Language, ModelChoice, SpeakItem, VoiceSample};
use futures_util::StreamExt;

fn item(text: &str) -> SpeakItem {
    SpeakItem {
        text: text.into(),
        language: Language::Ja,
        voice: "美咲".into(),
    }
}

fn samples() -> Vec<VoiceSample> {
    vec![VoiceSample {
        voice: "美咲".into(),
        language: Language::Ja,
        audio: vec![1],
        text: "はじめまして。".into(),
    }]
}

async fn first_error(service: &AiService) -> String {
    let spoken: Vec<_> = service
        .speak_lines(vec![item("おはよう。"), item("またね。")], samples())
        .collect()
        .await;
    assert_eq!(spoken.len(), 1);
    spoken[0].as_ref().unwrap_err().to_string()
}

#[tokio::test]
async fn without_a_model_picked_nothing_runs() {
    let dir = tempfile::tempdir().unwrap();
    let service = AiService::open(&dir.path().join("ai")).unwrap();
    assert_eq!(first_error(&service).await, "未选择合成台词模型");
    let designed = service
        .design_voice("明亮的声音。", Language::Ja, "おはよう。", 7)
        .await;
    assert_eq!(designed.unwrap_err().to_string(), "未选择生成参考音模型");
}

#[tokio::test]
async fn without_machines_the_first_item_fails_and_ends_it() {
    let dir = tempfile::tempdir().unwrap();
    let service = AiService::open(&dir.path().join("ai")).unwrap();
    service
        .update_settings(AiSettings {
            models: ModelChoice {
                speak: Some("Qwen3-TTS-12Hz-1.7B-Base".into()),
                design_voice: None,
            },
            ..AiSettings::default()
        })
        .unwrap();
    assert_eq!(
        first_error(&service).await,
        "没有能运行 Qwen3-TTS-12Hz-1.7B-Base 的机器"
    );
}
