//! A script is checked together with where it goes: a world that can take a
//! new episode, or an episode that is still there.

use crate::support::{SAMPLE_SCRIPT, main_role, sample_episode, seeded, service};

#[tokio::test]
async fn a_passing_script_for_a_complete_world_has_no_problems() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;

    let problems = listening
        .check_import_script("星见町", SAMPLE_SCRIPT)
        .await
        .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
}

#[tokio::test]
async fn script_and_world_problems_come_together() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;

    let problems = listening.check_import_script("火星", "{}").await.unwrap();
    assert_eq!(
        problems,
        [
            "缺少 title",
            "缺少 summary",
            "缺少 lines",
            "世界 火星 不存在"
        ]
    );
}

#[tokio::test]
async fn a_world_without_a_narrator_cannot_take_a_script() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    listening
        .create_world("雾都", "雾很重的城市。")
        .await
        .unwrap();
    listening
        .add_cast("雾都", "美咲", &main_role("侦探。"))
        .await
        .unwrap();

    let problems = listening
        .check_import_script("雾都", SAMPLE_SCRIPT)
        .await
        .unwrap();
    assert_eq!(problems.len(), 1);
    assert!(problems[0].starts_with("世界 雾都 缺少"), "{problems:?}");
}

#[tokio::test]
async fn continuing_needs_the_episode() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let episode = sample_episode(&listening).await;

    let problems = listening
        .check_append_script(episode, SAMPLE_SCRIPT)
        .await
        .unwrap();
    assert!(problems.is_empty(), "{problems:?}");

    listening.delete_episode(episode).await.unwrap();
    let problems = listening
        .check_append_script(episode, SAMPLE_SCRIPT)
        .await
        .unwrap();
    assert_eq!(problems, [format!("要续写的集 {episode} 不存在")]);
}

#[tokio::test]
async fn continuing_needs_the_world_to_stay_complete() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let episode = sample_episode(&listening).await;
    listening.remove_cast("星见町", "老陈").await.unwrap();

    let problems = listening.check_append_script(episode, "{}").await.unwrap();
    assert_eq!(problems.len(), 4, "{problems:?}");
    assert!(problems[3].starts_with("世界 星见町 缺少"), "{problems:?}");
}
