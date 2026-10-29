//! The ai commands: speaking an episode's lines as a task, cancelling it, and
//! designing a voice. No GPU machine: the jobs fail for want of one, or wait
//! on a machine that never answers. ai-pipeline tests the jobs themselves.

use listening::{Language, Uuid};
use noobenkyo_core::commands::ai::speak_lines::ai_start_speak_lines;
use noobenkyo_core::commands::ai::tasks::ai_cancel;
use noobenkyo_core::commands::ai::voice::ai_design_voice;
use noobenkyo_core::events::ai_task_finished::{AiTaskFinished, TaskOutcome};
use noobenkyo_core::events::ai_task_progress::AiTaskProgress;

use crate::support::TestApp;

fn failed(message: &str) -> TaskOutcome {
    TaskOutcome::Failed {
        message: message.into(),
    }
}

async fn missing(app: &TestApp, episode: Uuid) -> u32 {
    let items = app.state().listening.missing_audio(episode).await.unwrap();
    items.len() as u32
}

#[tokio::test]
async fn without_machines_synthesis_fails_and_lets_the_episode_go() {
    let (app, episode) = TestApp::with_world().await;
    let total = missing(&app, episode).await;
    assert!(total > 0);
    let mut events = app.task_events();

    let task = ai_start_speak_lines(app.handle(), episode).await.unwrap();
    assert_eq!(
        events.progress().await,
        AiTaskProgress {
            task,
            done: 0,
            total
        }
    );
    assert_eq!(
        events.finished().await,
        AiTaskFinished {
            task,
            outcome: failed("没有能运行 qwen3-tts-1.7b-base 的机器"),
        }
    );

    let again = ai_start_speak_lines(app.handle(), episode).await.unwrap();
    assert_ne!(again, task);
    assert_eq!(events.finished().await.task, again);
}

#[tokio::test]
async fn an_episode_is_synthesized_once_at_a_time_until_cancelled() {
    let (app, episode) = TestApp::with_world().await;
    app.with_hanging_machine();
    let total = missing(&app, episode).await;
    let mut events = app.task_events();

    let task = ai_start_speak_lines(app.handle(), episode).await.unwrap();
    assert_eq!(events.progress().await.done, 0);
    let refused = ai_start_speak_lines(app.handle(), episode)
        .await
        .unwrap_err();
    assert_eq!(refused.message, "这一集正在合成");

    ai_cancel(app.state(), task).await.unwrap();
    assert_eq!(
        events.finished().await,
        AiTaskFinished {
            task,
            outcome: TaskOutcome::Cancelled,
        }
    );
    assert_eq!(missing(&app, episode).await, total);

    let again = ai_start_speak_lines(app.handle(), episode).await.unwrap();
    ai_cancel(app.state(), again).await.unwrap();
    assert_eq!(events.finished().await.outcome, TaskOutcome::Cancelled);
}

#[tokio::test]
async fn cancelling_a_task_that_has_ended_does_nothing() {
    let (app, episode) = TestApp::with_world().await;
    let mut events = app.task_events();
    let task = ai_start_speak_lines(app.handle(), episode).await.unwrap();
    events.finished().await;
    ai_cancel(app.state(), task).await.unwrap();
}

#[tokio::test]
async fn an_episode_that_is_gone_fails_the_task() {
    let app = TestApp::fresh().await;
    let mut events = app.task_events();
    let task = ai_start_speak_lines(app.handle(), Uuid::from_u128(7))
        .await
        .unwrap();
    assert_eq!(
        events.finished().await,
        AiTaskFinished {
            task,
            outcome: failed("这一集不存在"),
        }
    );
}

#[tokio::test]
async fn designing_a_voice_needs_a_machine() {
    let app = TestApp::fresh().await;
    let designed = ai_design_voice(
        app.state(),
        "20岁的女性，声音明亮。".into(),
        Language::Ja,
        "はじめまして。".into(),
        7,
    )
    .await;
    assert_eq!(
        designed.unwrap_err().message,
        "没有能运行 qwen3-tts-1.7b-voicedesign 的机器"
    );
}

/// Real synthesis on the machines in ai-invoker's test settings. Every
/// character gets the same two made-up references. Run it alone:
///
/// NOOBENKYO_INVOKER_TEST_SETTINGS=<file> cargo test -p noobenkyo-core -- --ignored --nocapture
#[tokio::test]
#[ignore = "needs a GPU machine"]
async fn synthesis_stores_each_recording_as_it_comes() {
    const PROMPT: &str = "20岁的女性，声音明亮，语速适中。";
    const JA: &str = "はじめまして。今日はいい天気ですね。一緒に散歩しませんか。";
    const ZH: &str = "你好，初次见面。今天天气真好，要不要一起去散散步？";
    let app = TestApp::fresh().await;
    app.with_gpu_machines();
    let ja = ai_design_voice(app.state(), PROMPT.into(), Language::Ja, JA.into(), 7)
        .await
        .unwrap();
    let zh = ai_design_voice(app.state(), PROMPT.into(), Language::Zh, ZH.into(), 7)
        .await
        .unwrap();
    let episode = app
        .seed_world(|language| match language {
            Language::Ja => listening::NewReference {
                language,
                audio: ja.clone(),
                text: JA.into(),
            },
            Language::Zh => listening::NewReference {
                language,
                audio: zh.clone(),
                text: ZH.into(),
            },
        })
        .await;
    let total = missing(&app, episode).await;
    let mut events = app.task_events();

    let task = ai_start_speak_lines(app.handle(), episode).await.unwrap();
    assert_eq!(events.progress().await.total, total);
    assert_eq!(events.progress_within(300).await.done, 1);
    assert_eq!(events.progress_within(60).await.done, 2);
    ai_cancel(app.state(), task).await.unwrap();
    assert_eq!(
        events.finished_within(60).await.outcome,
        TaskOutcome::Cancelled
    );
    let kept = total - missing(&app, episode).await;
    assert!((2..=3).contains(&kept), "{kept} kept");

    let again = ai_start_speak_lines(app.handle(), episode).await.unwrap();
    assert_eq!(events.progress().await.total, total - kept);
    ai_cancel(app.state(), again).await.unwrap();
    assert_eq!(
        events.finished_within(60).await.outcome,
        TaskOutcome::Cancelled
    );
}
