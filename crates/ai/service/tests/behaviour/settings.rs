//! Reading, replacing and refusing the settings.

use std::pin::pin;
use std::time::Duration;

use ai::{AiError, AiService, AiSettings, InvokerSettings, Language};

use crate::support::{hanging_machine, write_settings_file};

#[test]
fn without_a_file_there_are_no_machines() {
    let dir = tempfile::tempdir().unwrap();
    let service = AiService::open(&dir.path().join("ai")).unwrap();
    assert_eq!(service.settings(), AiSettings::default());
    assert!(service.settings().invoker.machines.is_empty());
}

#[test]
fn fields_missing_from_the_file_take_their_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let ai_dir = dir.path().join("ai");
    write_settings_file(&ai_dir, r#"{ "invoker": { "idle_timeout_secs": 60 } }"#);
    let service = AiService::open(&ai_dir).unwrap();
    let invoker = service.settings().invoker;
    assert_eq!(invoker.idle_timeout_secs, 60);
    assert_eq!(
        invoker.connect_timeout_secs,
        InvokerSettings::default().connect_timeout_secs
    );
}

#[tokio::test]
async fn a_broken_file_is_named_by_every_call_until_settings_replace_it() {
    let dir = tempfile::tempdir().unwrap();
    let ai_dir = dir.path().join("ai");
    write_settings_file(&ai_dir, r#"{ "invoker": { "idle_timeout_secs": 0 } }"#);
    let service = AiService::open(&ai_dir).unwrap();
    assert_eq!(service.settings(), AiSettings::default());

    let designed = service
        .design_voice("明亮的声音。", Language::Ja, "おはよう。", 7)
        .await;
    let error = designed.unwrap_err();
    assert!(matches!(error, AiError::BadSettingsFile(_)), "{error:?}");
    assert_eq!(
        error.to_string(),
        "ai/settings.json 没有用上：闲置超时至少 1 秒"
    );

    service.update_settings(AiSettings::default()).unwrap();
    let designed = service
        .design_voice("明亮的声音。", Language::Ja, "おはよう。", 7)
        .await;
    // The file is no longer named: the call gets as far as picking a model.
    assert_eq!(designed.unwrap_err().to_string(), "未选择生成参考音模型");
}

#[test]
fn new_settings_are_kept_in_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let ai_dir = dir.path().join("ai");
    let service = AiService::open(&ai_dir).unwrap();
    let new = hanging_machine(dir.path());
    service.update_settings(new.clone()).unwrap();
    assert_eq!(service.settings(), new);
    drop(service);
    assert_eq!(AiService::open(&ai_dir).unwrap().settings(), new);
}

#[test]
fn invalid_settings_are_rejected_and_the_old_ones_kept() {
    let dir = tempfile::tempdir().unwrap();
    let ai_dir = dir.path().join("ai");
    let service = AiService::open(&ai_dir).unwrap();
    let mut invalid = AiSettings::default();
    invalid.invoker.connect_timeout_secs = 0;
    let error = service.update_settings(invalid).unwrap_err();
    assert_eq!(error.to_string(), "连接超时至少 1 秒");
    assert_eq!(service.settings(), AiSettings::default());
    assert!(!ai_dir.join("settings.json").exists());
}

#[tokio::test]
async fn settings_cannot_change_under_a_running_call() {
    let dir = tempfile::tempdir().unwrap();
    let service = AiService::open(&dir.path().join("ai")).unwrap();
    service
        .update_settings(hanging_machine(dir.path()))
        .unwrap();

    {
        let mut designing =
            pin!(service.design_voice("明亮的声音。", Language::Ja, "おはよう。", 7));
        tokio::select! {
            designed = &mut designing => panic!("finished: {designed:?}"),
            _ = tokio::time::sleep(Duration::from_millis(300)) => {}
        }
        let refused = service.update_settings(AiSettings::default());
        assert!(matches!(refused, Err(AiError::Busy)), "{refused:?}");
    }
    // The call is dropped with its block.
    service.update_settings(AiSettings::default()).unwrap();
}
