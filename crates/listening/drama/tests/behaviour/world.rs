use listening_drama::world::store::create::create_world;
use listening_drama::world::store::delete::delete_world;
use listening_drama::world::store::detail::world;
use listening_drama::world::store::list::worlds;
use listening_drama::world::store::setting::update_setting;
use listening_drama::{DramaError, WorldDetail, WorldSummary};

use crate::support::{first_script, import, memory_db, seeded_world};

#[tokio::test]
async fn created_worlds_are_listed_in_order_with_episode_counts() {
    let db = memory_db().await;
    seeded_world(&db).await;
    create_world(&db, " 月影岛 ", " ").await.unwrap();
    import(&db, &first_script(), 1).await;
    import(&db, &first_script(), 2).await;

    assert_eq!(
        worlds(&db).await.unwrap(),
        vec![
            WorldSummary {
                name: "星见町".into(),
                episodes: 2,
            },
            WorldSummary {
                name: "月影岛".into(),
                episodes: 0,
            },
        ]
    );
    assert_eq!(
        world(&db, "月影岛").await.unwrap(),
        Some(WorldDetail {
            name: "月影岛".into(),
            setting: String::new(),
            cast: vec![],
        })
    );
    assert_eq!(world(&db, "雾都").await.unwrap(), None);
}

#[tokio::test]
async fn names_are_unique_and_not_blank() {
    let db = memory_db().await;
    create_world(&db, "星见町", "").await.unwrap();

    let again = create_world(&db, " 星见町 ", "").await.unwrap_err();
    assert_eq!(again.to_string(), "世界 星见町 已经存在");
    let blank = create_world(&db, " ", "").await.unwrap_err();
    assert!(matches!(blank, DramaError::EmptyWorldName));
}

#[tokio::test]
async fn the_setting_can_change() {
    let db = memory_db().await;
    create_world(&db, "星见町", "海边的小镇。").await.unwrap();

    update_setting(&db, "星见町", " 山间的小镇。 ")
        .await
        .unwrap();
    assert_eq!(
        world(&db, "星见町").await.unwrap().unwrap().setting,
        "山间的小镇。"
    );
    let missing = update_setting(&db, "雾都", "").await.unwrap_err();
    assert_eq!(missing.to_string(), "世界 雾都 不存在");
}

#[tokio::test]
async fn deleting_a_world() {
    let db = memory_db().await;
    seeded_world(&db).await;
    import(&db, &first_script(), 1).await;

    assert!(delete_world(&db, "星见町").await.unwrap());
    assert!(!delete_world(&db, "星见町").await.unwrap());
    assert!(worlds(&db).await.unwrap().is_empty());
}
