//! Constraints of the tables themselves, checked by writing rows directly.

use listening_drama::Language;
use listening_drama::character::store::delete::delete_character;
use listening_drama::entity::{cast, character, episode, line, line_audio, reference};
use listening_drama::line::store::delete::delete_lines;
use listening_drama::line::store::save_audio::save_audio;
use listening_drama::script::store::append::append_script;
use listening_drama::world::store::delete::delete_world;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::support::{first_script, import, lines, memory_db, rng, script, seeded_world};

fn character_row(name: &str) -> character::ActiveModel {
    character::ActiveModel {
        name: Set(name.into()),
        kana: Set("かな".into()),
        prompt: Set(None),
        gender: Set("female".into()),
        age: Set(20),
        ..Default::default()
    }
}

fn cast_row(world_id: i64, character_id: i64, role: &str) -> cast::ActiveModel {
    cast::ActiveModel {
        world_id: Set(world_id),
        character_id: Set(character_id),
        role: Set(role.into()),
        description: Set(None),
    }
}

#[tokio::test]
async fn character_names_are_unique() {
    let db = memory_db().await;
    character_row("美咲").insert(&db).await.unwrap();
    assert!(character_row("美咲").insert(&db).await.is_err());
}

#[tokio::test]
async fn a_world_has_at_most_one_narrator_row() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let extra = character_row("真理").insert(&db).await.unwrap();
    let another = character_row("雪").insert(&db).await.unwrap();
    let world_id = cast::Entity::find()
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .world_id;

    assert!(
        cast_row(world_id, extra.id, "narrator")
            .insert(&db)
            .await
            .is_err()
    );
    cast_row(world_id, extra.id, "minor")
        .insert(&db)
        .await
        .unwrap();
    cast_row(world_id, another.id, "minor")
        .insert(&db)
        .await
        .unwrap();
}

#[tokio::test]
async fn line_ids_are_never_reused() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;
    let last = lines(&db, uuid).await[2].id;
    delete_lines(&db, &[last]).await.unwrap();
    append_script(
        &db,
        uuid,
        &script("续", &[("旁白", "あ", "啊")]),
        &mut rng(),
    )
    .await
    .unwrap();

    assert!(lines(&db, uuid).await[2].id > last);
}

#[tokio::test]
async fn deleting_a_world_takes_its_cast_episodes_lines_and_audio() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;
    save_audio(&db, lines(&db, uuid).await[0].id, Language::Ja, b"wav")
        .await
        .unwrap();

    delete_world(&db, "星见町").await.unwrap();
    assert_eq!(cast::Entity::find().count(&db).await.unwrap(), 0);
    assert_eq!(episode::Entity::find().count(&db).await.unwrap(), 0);
    assert_eq!(line::Entity::find().count(&db).await.unwrap(), 0);
    assert_eq!(line_audio::Entity::find().count(&db).await.unwrap(), 0);
    assert_eq!(character::Entity::find().count(&db).await.unwrap(), 4);
}

#[tokio::test]
async fn deleting_a_character_leaves_the_lines_it_voiced() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;

    delete_character(&db, "老陈").await.unwrap();
    assert_eq!(cast::Entity::find().count(&db).await.unwrap(), 3);
    assert_eq!(reference::Entity::find().count(&db).await.unwrap(), 5);
    assert_eq!(lines(&db, uuid).await[0].voice, "老陈");
}
