use listening::{Language, ListeningError, Reference, Role};

use crate::support::{new_character, seeded, service};

#[tokio::test]
async fn characters_are_created_listed_and_completed() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    let problems = listening
        .create_character(&new_character("美咲", "みさき", &[Language::Ja]))
        .await
        .unwrap();
    assert!(problems.is_empty());

    listening
        .add_reference("美咲", Language::Zh, b"zh.wav", "你好。")
        .await
        .unwrap();
    let listed = listening.characters().await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].languages, vec![Language::Ja, Language::Zh]);
    assert_eq!(
        listening.reference("美咲", Language::Zh).await.unwrap(),
        Some(Reference {
            audio: b"zh.wav".to_vec(),
            text: "你好。".into(),
        })
    );
    let detail = listening.character("美咲").await.unwrap().unwrap();
    assert_eq!(detail.references.len(), 2);
}

#[tokio::test]
async fn rejected_characters_come_back_as_problems() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;

    let problems = listening
        .create_character(&new_character("ミサキ", "みさき", &[]))
        .await
        .unwrap();
    assert_eq!(
        problems,
        vec!["名字里没有汉字，读音要和名字相同", "至少要有一段参考音"]
    );
    assert!(listening.characters().await.unwrap().is_empty());
}

#[tokio::test]
async fn deleting_a_character_is_confirmed_with_its_worlds() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;

    assert_eq!(
        listening.character_usage("店员").await.unwrap(),
        vec!["星见町"]
    );
    assert!(listening.delete_character("店员").await.unwrap());
    let cast = listening.world("星见町").await.unwrap().unwrap().cast;
    assert!(cast.iter().all(|member| member.role != Role::Minor));
    let error = listening.reference("店员", Language::Ja).await.unwrap_err();
    assert!(matches!(error, ListeningError::Drama(_)));
    assert_eq!(error.to_string(), "角色 店员 不存在");
}
