use std::collections::HashMap;

use listening_drama::character::store::delete::delete_character;
use listening_drama::episode::store::detail::episode;
use listening_drama::script::store::append::append_script;
use listening_drama::script::store::context::script_context;
use listening_drama::script::store::import::import_script;
use listening_drama::world::store::cast::add::add_cast;
use listening_drama::world::store::cast::set_role::set_role;
use listening_drama::world::store::create::create_world;
use listening_drama::{
    DramaError, Gender, LineView, MainCharacter, Role, ScriptEpisode, ScriptLine,
};
use uuid::Uuid;

use crate::support::{
    create, first_script, import, lines, main_role, memory_db, new_character, now, rng, script,
    seeded_world,
};

fn voices(lines: &[LineView]) -> Vec<(&str, &str)> {
    lines
        .iter()
        .map(|line| (line.speaker.as_str(), line.voice.as_str()))
        .collect()
}

#[tokio::test]
async fn importing_maps_every_speaker() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;

    assert_eq!(
        voices(&lines(&db, uuid).await),
        vec![("旁白", "老陈"), ("美咲", "美咲"), ("路人", "店员")]
    );
}

#[tokio::test]
async fn other_names_use_the_narrator_without_minor_characters() {
    let db = memory_db().await;
    seeded_world(&db).await;
    set_role(&db, "星见町", "店员", &main_role("便利店的店员。"))
        .await
        .unwrap();
    let uuid = import(&db, &first_script(), 1).await;

    assert_eq!(voices(&lines(&db, uuid).await)[2], ("路人", "老陈"));
}

#[tokio::test]
async fn names_keep_one_voice_and_spread_over_minor_characters() {
    let db = memory_db().await;
    seeded_world(&db).await;
    create(&db, &new_character("真理", "まり")).await;
    add_cast(&db, "星见町", "真理", &Role::Minor).await.unwrap();
    let uuid = import(
        &db,
        &script(
            "二",
            &[
                ("路人", "あ", "啊"),
                ("警察", "い", "咿"),
                ("路人", "う", "呜"),
            ],
        ),
        1,
    )
    .await;

    let stored = lines(&db, uuid).await;
    let mapped: HashMap<&str, &str> = voices(&stored).into_iter().collect();
    assert_eq!(mapped.len(), 2);
    let mut used: Vec<&str> = mapped.values().copied().collect();
    used.sort();
    assert_eq!(used, vec!["店员", "真理"]);
}

#[tokio::test]
async fn importing_needs_a_complete_cast() {
    let db = memory_db().await;
    create(&db, &new_character("美咲", "みさき")).await;
    create_world(&db, "月影岛", "").await.unwrap();
    add_cast(&db, "月影岛", "美咲", &Role::Minor).await.unwrap();

    let error = import_script(
        &db,
        "月影岛",
        &first_script(),
        Uuid::from_u128(1),
        now(),
        &mut rng(),
    )
    .await
    .unwrap_err();
    assert_eq!(error.to_string(), "世界 月影岛 缺少旁白、主要角色");
    let error = import_script(
        &db,
        "雾都",
        &first_script(),
        Uuid::from_u128(1),
        now(),
        &mut rng(),
    )
    .await
    .unwrap_err();
    assert!(matches!(error, DramaError::WorldNotFound(_)));
}

#[tokio::test]
async fn appending_adds_lines_and_replaces_the_summary() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;
    let before = lines(&db, uuid).await;

    let mut more = script("不用的标题", &[("美咲", "またね。", "再见。")]);
    more.summary = "两人相遇又道别。".into();
    append_script(&db, uuid, &more, &mut rng()).await.unwrap();

    let detail = episode(&db, uuid).await.unwrap().unwrap();
    assert_eq!(detail.title, "初遇");
    assert_eq!(detail.summary, "两人相遇又道别。");
    assert_eq!(detail.lines[..3], before[..]);
    assert_eq!(detail.lines[3].ja, "またね。");
}

#[tokio::test]
async fn appending_keeps_the_voices_the_episode_has() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;
    set_role(&db, "星见町", "老陈", &Role::Minor).await.unwrap();
    set_role(&db, "星见町", "店员", &Role::Narrator)
        .await
        .unwrap();
    delete_character(&db, "美咲").await.unwrap();
    create(&db, &new_character("真理", "まり")).await;
    add_cast(&db, "星见町", "真理", &main_role("转学生。"))
        .await
        .unwrap();

    let more = script(
        "续",
        &[
            ("旁白", "{夜|よる}。", "夜晚。"),
            ("美咲", "ただいま。", "我回来了。"),
            ("真理", "おかえり。", "欢迎回来。"),
        ],
    );
    append_script(&db, uuid, &more, &mut rng()).await.unwrap();

    assert_eq!(
        voices(&lines(&db, uuid).await)[3..],
        [("旁白", "老陈"), ("美咲", "美咲"), ("真理", "真理")]
    );
}

#[tokio::test]
async fn appending_needs_the_episode_and_a_complete_cast() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;

    let missing = append_script(&db, Uuid::from_u128(9), &first_script(), &mut rng()).await;
    assert!(matches!(missing, Err(DramaError::EpisodeNotFound)));
    set_role(&db, "星见町", "老陈", &Role::Minor).await.unwrap();
    let incomplete = append_script(&db, uuid, &first_script(), &mut rng())
        .await
        .unwrap_err();
    assert_eq!(incomplete.to_string(), "世界 星见町 缺少旁白");
    assert_eq!(lines(&db, uuid).await.len(), 3);
}

#[tokio::test]
async fn context_has_the_world_its_main_characters_and_the_chosen_episodes() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let first = import(&db, &first_script(), 1).await;
    let second = import(&db, &script("二", &[("健二", "よう。", "哟。")]), 2).await;
    let third = import(&db, &script("三", &[("美咲", "ね。", "呐。")]), 3).await;

    let context = script_context(&db, "星见町", &[second, first], Some(third))
        .await
        .unwrap();
    assert_eq!(context.world, "星见町");
    assert_eq!(context.setting, "海边的小镇。");
    assert_eq!(
        context.mains,
        vec![
            MainCharacter {
                name: "美咲".into(),
                kana: "みさき".into(),
                gender: Gender::Female,
                age: 20,
                description: "高中生，开朗。".into(),
            },
            MainCharacter {
                name: "健二".into(),
                kana: "けんじ".into(),
                gender: Gender::Female,
                age: 20,
                description: "美咲的同学。".into(),
            },
        ]
    );
    let titles: Vec<&str> = context
        .predecessors
        .iter()
        .map(|episode| episode.title.as_str())
        .collect();
    assert_eq!(titles, vec!["二", "初遇"]);
    assert_eq!(
        context.appending,
        Some(ScriptEpisode {
            number: 3.0,
            title: "三".into(),
            summary: "三的梗概。".into(),
            lines: vec![ScriptLine {
                speaker: "美咲".into(),
                ja: "ね。".into(),
                zh: "呐。".into(),
            }],
        })
    );
    assert_eq!(
        context.predecessors[1].lines[0].ja,
        "{春|はる}の{朝|あさ}。"
    );
}

#[tokio::test]
async fn context_needs_a_complete_cast_and_episodes_of_the_world() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let first = import(&db, &first_script(), 1).await;
    create_world(&db, "月影岛", "").await.unwrap();
    add_cast(&db, "月影岛", "老陈", &Role::Narrator)
        .await
        .unwrap();

    let error = script_context(&db, "月影岛", &[], None).await.unwrap_err();
    assert_eq!(error.to_string(), "世界 月影岛 缺少主要角色");
    add_cast(&db, "月影岛", "美咲", &main_role("岛上的医生。"))
        .await
        .unwrap();
    let error = script_context(&db, "月影岛", &[first], None)
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "集 初遇 不在世界 月影岛 里");
    let error = script_context(&db, "月影岛", &[], Some(Uuid::from_u128(9)))
        .await
        .unwrap_err();
    assert!(matches!(error, DramaError::EpisodeNotFound));
    assert!(script_context(&db, "月影岛", &[], None).await.is_ok());
}
