use listening::{ListeningError, check_script};

use crate::support::{SAMPLE_SCRIPT, sample_episode, seeded, service};

#[test]
fn the_sample_script_passes() {
    assert!(check_script(SAMPLE_SCRIPT).is_empty());
    assert_eq!(
        check_script("{}"),
        vec!["缺少 title", "缺少 summary", "缺少 lines"]
    );
}

#[tokio::test]
async fn importing_the_sample_script() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let uuid = sample_episode(&listening).await;

    let detail = listening.episode(uuid).await.unwrap().unwrap();
    assert_eq!(detail.world, "星见町");
    assert_eq!(detail.number, 1.0);
    assert_eq!(detail.title, "海边的早晨");
    let voices: Vec<(&str, &str)> = detail
        .lines
        .iter()
        .map(|line| (line.speaker.as_str(), line.voice.as_str()))
        .collect();
    assert_eq!(
        voices,
        vec![
            ("旁白", "老陈"),
            ("店员", "店员"),
            ("美咲", "美咲"),
            ("健二", "健二"),
            ("美咲", "美咲"),
            ("健二", "健二"),
            ("路人", "店员"),
            ("旁白", "老陈"),
        ]
    );
    assert_eq!(
        detail.lines[0].ja,
        "{潮風|しおかぜ}が{吹|ふ}く、{春|はる}の{朝|あさ}。"
    );
}

#[tokio::test]
async fn a_failing_script_is_not_stored() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;

    let text = r#"{"title":"初遇","summary":"","lines":[{"speaker":"旁白","ja":"{朝|asa}","zh":"早晨。"}]}"#;
    let error = listening.import_script("星见町", text).await.unwrap_err();
    assert!(matches!(error, ListeningError::Script(_)));
    assert_eq!(
        error.to_string(),
        "剧本不通过：summary 为空；第 1 行 ja：读音标记 {朝|asa} 的读音只能有假名，不能有 a"
    );
    assert!(listening.episodes("星见町").await.unwrap().is_empty());
}

#[tokio::test]
async fn appending_continues_the_episode() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let uuid = sample_episode(&listening).await;

    let more = r#"{
        "title": "不用",
        "summary": "两人约好看灯塔，傍晚在灯塔下重逢。",
        "lines": [
            { "speaker": "旁白", "ja": "{夕方|ゆうがた}。", "zh": "傍晚。" },
            { "speaker": "路人", "ja": "ほら、きれいでしょう。", "zh": "你看，很美吧。" }
        ]
    }"#;
    listening.append_script(uuid, more).await.unwrap();

    let detail = listening.episode(uuid).await.unwrap().unwrap();
    assert_eq!(detail.title, "海边的早晨");
    assert_eq!(detail.summary, "两人约好看灯塔，傍晚在灯塔下重逢。");
    assert_eq!(detail.lines.len(), 10);
    assert_eq!(detail.lines[9].voice, "店员");
}

#[tokio::test]
async fn context_for_the_next_episode() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let uuid = sample_episode(&listening).await;

    let context = listening
        .script_context("星见町", &[uuid], None)
        .await
        .unwrap();
    assert_eq!(context.setting, "海边的小镇。");
    let mains: Vec<&str> = context
        .mains
        .iter()
        .map(|main| main.name.as_str())
        .collect();
    assert_eq!(mains, vec!["美咲", "健二"]);
    assert_eq!(context.predecessors[0].lines.len(), 8);
    assert_eq!(context.appending, None);

    listening.remove_cast("星见町", "老陈").await.unwrap();
    let error = listening
        .script_context("星见町", &[], None)
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "世界 星见町 缺少旁白");
}
