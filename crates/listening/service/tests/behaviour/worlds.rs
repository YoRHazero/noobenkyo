use listening::{Role, WorldSummary};

use crate::support::{main_role, sample_episode, seeded, service};

#[tokio::test]
async fn worlds_and_their_casts() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    sample_episode(&listening).await;

    assert_eq!(
        listening.worlds().await.unwrap(),
        vec![WorldSummary {
            name: "星见町".into(),
            episodes: 1,
        }]
    );
    listening
        .update_setting("星见町", "山间的小镇。")
        .await
        .unwrap();
    listening
        .set_role("星见町", "店员", &main_role("便利店的店员。"))
        .await
        .unwrap();
    assert!(listening.remove_cast("星见町", "健二").await.unwrap());

    let detail = listening.world("星见町").await.unwrap().unwrap();
    assert_eq!(detail.setting, "山间的小镇。");
    let cast: Vec<(&str, &Role)> = detail
        .cast
        .iter()
        .map(|member| (member.character.as_str(), &member.role))
        .collect();
    assert_eq!(
        cast,
        vec![
            ("老陈", &Role::Narrator),
            ("美咲", &main_role("高中生，喜欢拍照。")),
            ("店员", &main_role("便利店的店员。")),
        ]
    );
}

#[tokio::test]
async fn errors_carry_the_message_from_drama() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;

    let error = listening
        .add_cast("星见町", "店员", &Role::Narrator)
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "角色 店员 已经在世界 星见町 里");
    let error = listening
        .set_role("星见町", "店员", &Role::Narrator)
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "世界 星见町 已经有旁白 老陈");
}

#[tokio::test]
async fn deleting_a_world_takes_its_episodes() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let uuid = sample_episode(&listening).await;

    assert!(listening.delete_world("星见町").await.unwrap());
    assert_eq!(listening.episode(uuid).await.unwrap(), None);
    assert_eq!(listening.characters().await.unwrap().len(), 4);
}
