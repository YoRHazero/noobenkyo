use listening_drama::character::store::create::create_character;
use listening_drama::character::store::delete::delete_character;
use listening_drama::character::store::detail::character;
use listening_drama::character::store::languages::languages_by_name;
use listening_drama::character::store::list::characters;
use listening_drama::character::store::reference::{add_reference, reference};
use listening_drama::entity::reference as reference_table;
use listening_drama::{
    CharacterDetail, CharacterSummary, DramaError, Gender, Language, NewCharacter, Reference,
    ReferenceText,
};
use sea_orm::EntityTrait;

use crate::support::{create, memory_db, new_character, reference as new_reference};

#[tokio::test]
async fn created_characters_are_listed_in_order() {
    let db = memory_db().await;
    create(
        &db,
        &NewCharacter {
            prompt: Some(" 二十岁的女性。开朗。 ".into()),
            references: vec![
                new_reference(Language::Zh, "zh.wav", "你好。"),
                new_reference(Language::Ja, "ja.wav", "こんにちは。"),
            ],
            ..new_character(" 美咲 ", "みさき")
        },
    )
    .await;
    create(
        &db,
        &NewCharacter {
            gender: Gender::Male,
            age: 45,
            ..new_character("ケンジ", "ケンジ")
        },
    )
    .await;

    assert_eq!(
        characters(&db).await.unwrap(),
        vec![
            CharacterSummary {
                name: "美咲".into(),
                kana: "みさき".into(),
                gender: Gender::Female,
                age: 20,
                languages: vec![Language::Ja, Language::Zh],
                has_prompt: true,
            },
            CharacterSummary {
                name: "ケンジ".into(),
                kana: "ケンジ".into(),
                gender: Gender::Male,
                age: 45,
                languages: vec![Language::Ja],
                has_prompt: false,
            },
        ]
    );
}

#[tokio::test]
async fn detail_has_the_prompt_and_reference_texts() {
    let db = memory_db().await;
    create(
        &db,
        &NewCharacter {
            prompt: Some("二十岁的女性。".into()),
            ..new_character("美咲", "みさき")
        },
    )
    .await;

    assert_eq!(
        character(&db, "美咲").await.unwrap(),
        Some(CharacterDetail {
            name: "美咲".into(),
            kana: "みさき".into(),
            gender: Gender::Female,
            age: 20,
            prompt: Some("二十岁的女性。".into()),
            references: vec![ReferenceText {
                language: Language::Ja,
                text: "こんにちは。".into(),
            }],
        })
    );
    assert_eq!(character(&db, "健二").await.unwrap(), None);
}

#[tokio::test]
async fn rejected_characters_are_not_stored() {
    let db = memory_db().await;
    let problems = create_character(
        &db,
        &NewCharacter {
            age: 0,
            ..new_character("美咲", "みさき")
        },
    )
    .await
    .unwrap();
    assert_eq!(problems, vec!["年龄要在 1～100 之间"]);
    assert!(characters(&db).await.unwrap().is_empty());
}

#[tokio::test]
async fn a_character_can_start_without_references() {
    let db = memory_db().await;
    let problems = create_character(
        &db,
        &NewCharacter {
            references: vec![],
            ..new_character("美咲", "みさき")
        },
    )
    .await
    .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    let stored = character(&db, "美咲").await.unwrap().unwrap();
    assert!(stored.references.is_empty());
}

#[tokio::test]
async fn names_are_unique() {
    let db = memory_db().await;
    create(&db, &new_character("美咲", "みさき")).await;
    let problems = create_character(&db, &new_character("美咲", "みさき"))
        .await
        .unwrap();
    assert_eq!(problems, vec!["角色 美咲 已经存在"]);
    assert_eq!(characters(&db).await.unwrap().len(), 1);
}

#[tokio::test]
async fn a_missing_reference_can_be_added_once() {
    let db = memory_db().await;
    create(&db, &new_character("美咲", "みさき")).await;

    add_reference(&db, "美咲", Language::Zh, b"zh.wav", " 你好。 ")
        .await
        .unwrap();
    assert_eq!(
        reference(&db, "美咲", Language::Zh).await.unwrap(),
        Some(Reference {
            audio: b"zh.wav".to_vec(),
            text: "你好。".into(),
        })
    );

    let again = add_reference(&db, "美咲", Language::Zh, b"other.wav", "你好。").await;
    assert!(matches!(again, Err(DramaError::ReferenceExists { .. })));
    assert_eq!(again.unwrap_err().to_string(), "角色 美咲 已经有中文参考音");
    assert_eq!(
        reference(&db, "美咲", Language::Zh)
            .await
            .unwrap()
            .unwrap()
            .audio,
        b"zh.wav"
    );
}

#[tokio::test]
async fn adding_a_reference_checks_it_and_the_character() {
    let db = memory_db().await;
    create(&db, &new_character("美咲", "みさき")).await;

    let blank = add_reference(&db, "美咲", Language::Zh, b"", " ").await;
    assert_eq!(
        blank.unwrap_err().to_string(),
        "中文参考音的音频为空；中文参考音的文字为空"
    );
    let missing = add_reference(&db, "健二", Language::Zh, b"zh.wav", "你好。").await;
    assert!(matches!(missing, Err(DramaError::CharacterNotFound(_))));
}

#[tokio::test]
async fn reading_a_reference() {
    let db = memory_db().await;
    create(&db, &new_character("美咲", "みさき")).await;

    assert_eq!(
        reference(&db, "美咲", Language::Ja).await.unwrap(),
        Some(Reference {
            audio: b"ja.wav".to_vec(),
            text: "こんにちは。".into(),
        })
    );
    assert_eq!(reference(&db, "美咲", Language::Zh).await.unwrap(), None);
    assert!(matches!(
        reference(&db, "健二", Language::Ja).await,
        Err(DramaError::CharacterNotFound(_))
    ));
}

#[tokio::test]
async fn languages_by_name_leaves_out_missing_characters() {
    let db = memory_db().await;
    create(&db, &new_character("美咲", "みさき")).await;

    let found = languages_by_name(&db, &["美咲".into(), "健二".into()])
        .await
        .unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found["美咲"], vec![Language::Ja]);
}

#[tokio::test]
async fn deleting_a_character_takes_its_references() {
    let db = memory_db().await;
    create(&db, &new_character("美咲", "みさき")).await;

    assert!(delete_character(&db, "美咲").await.unwrap());
    assert!(!delete_character(&db, "美咲").await.unwrap());
    assert!(characters(&db).await.unwrap().is_empty());
    assert!(
        reference_table::Entity::find()
            .all(&db)
            .await
            .unwrap()
            .is_empty()
    );
}
