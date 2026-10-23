use listening_drama::character::store::delete::delete_character;
use listening_drama::entity::cast as cast_table;
use listening_drama::world::store::cast::add::add_cast;
use listening_drama::world::store::cast::remove::remove_cast;
use listening_drama::world::store::cast::set_role::set_role;
use listening_drama::world::store::cast::usage::character_usage;
use listening_drama::world::store::create::create_world;
use listening_drama::world::store::detail::world;
use listening_drama::{CastMember, DramaError, Language, Role};
use sea_orm::EntityTrait;

use crate::support::{create, main_role, memory_db, new_character, seeded_world};

async fn cast_of(db: &sea_orm::DatabaseConnection, name: &str) -> Vec<(String, Role)> {
    world(db, name)
        .await
        .unwrap()
        .unwrap()
        .cast
        .into_iter()
        .map(|member| (member.character, member.role))
        .collect()
}

#[tokio::test]
async fn cast_lists_narrator_then_main_then_minor() {
    let db = memory_db().await;
    seeded_world(&db).await;

    let detail = world(&db, "星见町").await.unwrap().unwrap();
    assert_eq!(
        detail.cast,
        vec![
            CastMember {
                character: "老陈".into(),
                role: Role::Narrator,
                languages: vec![Language::Ja, Language::Zh],
            },
            CastMember {
                character: "美咲".into(),
                role: main_role("高中生，开朗。"),
                languages: vec![Language::Ja, Language::Zh],
            },
            CastMember {
                character: "健二".into(),
                role: main_role("美咲的同学。"),
                languages: vec![Language::Ja],
            },
            CastMember {
                character: "店员".into(),
                role: Role::Minor,
                languages: vec![Language::Ja, Language::Zh],
            },
        ]
    );
}

#[tokio::test]
async fn a_world_has_one_narrator() {
    let db = memory_db().await;
    seeded_world(&db).await;

    create(&db, &new_character("真理", "まり")).await;
    let error = add_cast(&db, "星见町", "真理", &Role::Narrator)
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "世界 星见町 已经有旁白 老陈");
    let error = set_role(&db, "星见町", "店员", &Role::Narrator)
        .await
        .unwrap_err();
    assert!(matches!(error, DramaError::NarratorTaken { .. }));

    set_role(&db, "星见町", "老陈", &Role::Narrator)
        .await
        .unwrap();
    set_role(&db, "星见町", "老陈", &Role::Minor).await.unwrap();
    set_role(&db, "星见町", "店员", &Role::Narrator)
        .await
        .unwrap();
    assert_eq!(
        cast_of(&db, "星见町").await[0],
        ("店员".into(), Role::Narrator)
    );
}

#[tokio::test]
async fn adding_checks_the_world_the_character_and_the_role() {
    let db = memory_db().await;
    seeded_world(&db).await;
    create(&db, &new_character("真理", "まり")).await;

    let twice = add_cast(&db, "星见町", "美咲", &Role::Minor)
        .await
        .unwrap_err();
    assert_eq!(twice.to_string(), "角色 美咲 已经在世界 星见町 里");
    let no_world = add_cast(&db, "雾都", "真理", &Role::Minor)
        .await
        .unwrap_err();
    assert!(matches!(no_world, DramaError::WorldNotFound(_)));
    let no_character = add_cast(&db, "星见町", "雪", &Role::Minor)
        .await
        .unwrap_err();
    assert!(matches!(no_character, DramaError::CharacterNotFound(_)));
    let blank = add_cast(&db, "星见町", "真理", &main_role(" "))
        .await
        .unwrap_err();
    assert_eq!(blank.to_string(), "主要角色的身份不能为空");

    add_cast(&db, "星见町", "真理", &main_role(" 转学生。 "))
        .await
        .unwrap();
    assert!(
        cast_of(&db, "星见町")
            .await
            .contains(&("真理".into(), main_role("转学生。")))
    );
}

#[tokio::test]
async fn a_role_changes_in_place() {
    let db = memory_db().await;
    seeded_world(&db).await;

    set_role(&db, "星见町", "美咲", &main_role("大学生。"))
        .await
        .unwrap();
    assert!(
        cast_of(&db, "星见町")
            .await
            .contains(&("美咲".into(), main_role("大学生。")))
    );

    set_role(&db, "星见町", "美咲", &Role::Minor).await.unwrap();
    let rows = cast_table::Entity::find().all(&db).await.unwrap();
    assert!(
        rows.iter()
            .all(|row| (row.role == "main") == row.description.is_some())
    );

    create(&db, &new_character("真理", "まり")).await;
    let outside = set_role(&db, "星见町", "真理", &Role::Minor)
        .await
        .unwrap_err();
    assert_eq!(outside.to_string(), "角色 真理 不在世界 星见町 里");
}

#[tokio::test]
async fn removing_from_the_cast() {
    let db = memory_db().await;
    seeded_world(&db).await;

    assert!(remove_cast(&db, "星见町", "店员").await.unwrap());
    assert!(!remove_cast(&db, "星见町", "店员").await.unwrap());
    assert!(!remove_cast(&db, "雾都", "店员").await.unwrap());
    assert_eq!(cast_of(&db, "星见町").await.len(), 3);
}

#[tokio::test]
async fn usage_lists_the_worlds_of_a_character() {
    let db = memory_db().await;
    seeded_world(&db).await;
    create_world(&db, "月影岛", "").await.unwrap();
    add_cast(&db, "月影岛", "美咲", &Role::Narrator)
        .await
        .unwrap();

    assert_eq!(
        character_usage(&db, "美咲").await.unwrap(),
        vec!["星见町", "月影岛"]
    );
    assert!(character_usage(&db, "雪").await.unwrap().is_empty());

    delete_character(&db, "美咲").await.unwrap();
    assert!(world(&db, "月影岛").await.unwrap().unwrap().cast.is_empty());
    assert_eq!(cast_of(&db, "星见町").await.len(), 3);
}
