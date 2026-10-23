use listening_drama::episode::store::delete::delete_episode;
use listening_drama::episode::store::detail::episode;
use listening_drama::episode::store::list::episodes;
use listening_drama::episode::store::update::update_episode;
use listening_drama::line::store::save_audio::save_audio;
use listening_drama::{DramaError, EpisodeSummary, Language, LineId, LineView};
use uuid::Uuid;

use crate::support::{first_script, import, lines, memory_db, now, script, seeded_world};

fn numbers(list: &[EpisodeSummary]) -> Vec<(f64, String)> {
    list.iter()
        .map(|episode| (episode.number, episode.title.clone()))
        .collect()
}

#[tokio::test]
async fn new_episodes_follow_the_largest_number() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let first = import(&db, &script("一", &[("旁白", "あ", "啊")]), 1).await;
    import(&db, &script("二", &[("旁白", "あ", "啊")]), 2).await;
    update_episode(&db, first, 2.5, "一", "梗概。")
        .await
        .unwrap();
    import(&db, &script("三", &[("旁白", "あ", "啊")]), 3).await;

    assert_eq!(
        numbers(&episodes(&db, "星见町").await.unwrap()),
        vec![(2.0, "二".into()), (2.5, "一".into()), (3.0, "三".into())]
    );
}

#[tokio::test]
async fn equal_numbers_keep_the_order_of_creation() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let first = import(&db, &script("一", &[("旁白", "あ", "啊")]), 1).await;
    let second = import(&db, &script("二", &[("旁白", "あ", "啊")]), 2).await;
    update_episode(&db, second, 1.0, "二", "梗概。")
        .await
        .unwrap();
    update_episode(&db, first, 1.0, "一", "梗概。")
        .await
        .unwrap();

    assert_eq!(
        numbers(&episodes(&db, "星见町").await.unwrap()),
        vec![(1.0, "一".into()), (1.0, "二".into())]
    );
}

#[tokio::test]
async fn the_list_counts_lines_characters_and_missing_audio() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;
    let ids: Vec<LineId> = lines(&db, uuid).await.iter().map(|line| line.id).collect();
    save_audio(&db, ids[0], Language::Ja, b"wav").await.unwrap();
    save_audio(&db, ids[1], Language::Ja, b"wav").await.unwrap();
    save_audio(&db, ids[1], Language::Zh, b"wav").await.unwrap();

    assert_eq!(
        episodes(&db, "星见町").await.unwrap(),
        vec![EpisodeSummary {
            uuid,
            number: 1.0,
            title: "初遇".into(),
            summary: "初遇的梗概。".into(),
            created_at: now(),
            lines: 3,
            // Characters, not bytes; reading marks count too.
            char_count: 47,
            missing_ja: 1,
            missing_zh: 2,
        }]
    );
    let missing = episodes(&db, "雾都").await.unwrap_err();
    assert!(matches!(missing, DramaError::WorldNotFound(_)));
}

#[tokio::test]
async fn detail_has_every_line_with_speaker_and_voice() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;

    let detail = episode(&db, uuid).await.unwrap().unwrap();
    assert_eq!(detail.world, "星见町");
    assert_eq!(detail.title, "初遇");
    assert_eq!(detail.summary, "初遇的梗概。");
    assert_eq!(detail.created_at, now());
    let first = &detail.lines[0];
    assert_eq!(
        *first,
        LineView {
            id: first.id,
            speaker: "旁白".into(),
            voice: "老陈".into(),
            ja: "{春|はる}の{朝|あさ}。".into(),
            zh: "春天的早晨。".into(),
            audio: vec![],
        }
    );
    assert_eq!(episode(&db, Uuid::from_u128(9)).await.unwrap(), None);
}

#[tokio::test]
async fn number_title_and_summary_change_together() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;

    let problems = update_episode(&db, uuid, f64::INFINITY, " ", "新梗概。")
        .await
        .unwrap();
    assert_eq!(problems, vec!["集数不是有限的数", "标题为空"]);
    assert_eq!(episode(&db, uuid).await.unwrap().unwrap().title, "初遇");

    let problems = update_episode(&db, uuid, 0.5, " 序章 ", " 新梗概。 ")
        .await
        .unwrap();
    assert!(problems.is_empty());
    let detail = episode(&db, uuid).await.unwrap().unwrap();
    assert_eq!(
        (
            detail.number,
            detail.title.as_str(),
            detail.summary.as_str()
        ),
        (0.5, "序章", "新梗概。")
    );

    let missing = update_episode(&db, Uuid::from_u128(9), 1.0, "一", "梗概。").await;
    assert!(matches!(missing, Err(DramaError::EpisodeNotFound)));
}

#[tokio::test]
async fn deleting_an_episode() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &first_script(), 1).await;

    assert!(delete_episode(&db, uuid).await.unwrap());
    assert!(!delete_episode(&db, uuid).await.unwrap());
    assert!(episodes(&db, "星见町").await.unwrap().is_empty());
}
