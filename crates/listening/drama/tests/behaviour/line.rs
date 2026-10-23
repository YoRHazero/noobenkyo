use listening_drama::character::store::delete::delete_character;
use listening_drama::line::store::audio::audio;
use listening_drama::line::store::clear_audio::clear_audio;
use listening_drama::line::store::delete::delete_lines;
use listening_drama::line::store::missing_audio::missing_audio;
use listening_drama::line::store::save_audio::save_audio;
use listening_drama::line::store::update::update_line;
use listening_drama::{DramaError, Language, LineId, SpeechItem};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::support::{first_script, import, lines, memory_db, script, seeded_world};

/// 星见町 with the first script stored; returns its uuid and line ids.
async fn stored(db: &DatabaseConnection) -> (Uuid, Vec<LineId>) {
    seeded_world(db).await;
    let uuid = import(db, &first_script(), 1).await;
    let ids = lines(db, uuid).await.iter().map(|line| line.id).collect();
    (uuid, ids)
}

#[tokio::test]
async fn changing_a_text_drops_its_audio_only() {
    let db = memory_db().await;
    let (uuid, ids) = stored(&db).await;
    save_audio(&db, ids[1], Language::Ja, b"ja").await.unwrap();
    save_audio(&db, ids[1], Language::Zh, b"zh").await.unwrap();

    let problems = update_line(&db, ids[1], Language::Ja, " {朝|あさ}だよ！ ")
        .await
        .unwrap();
    assert!(problems.is_empty());
    let line = &lines(&db, uuid).await[1];
    assert_eq!(line.ja, "{朝|あさ}だよ！");
    assert_eq!(line.audio, vec![Language::Zh]);
}

#[tokio::test]
async fn the_same_text_changes_nothing() {
    let db = memory_db().await;
    let (uuid, ids) = stored(&db).await;
    save_audio(&db, ids[1], Language::Ja, b"ja").await.unwrap();

    let problems = update_line(&db, ids[1], Language::Ja, " おはよう！ ")
        .await
        .unwrap();
    assert!(problems.is_empty());
    assert_eq!(lines(&db, uuid).await[1].audio, vec![Language::Ja]);
}

#[tokio::test]
async fn a_rejected_text_changes_nothing() {
    let db = memory_db().await;
    let (uuid, ids) = stored(&db).await;

    let problems = update_line(&db, ids[1], Language::Ja, "{朝|asa}")
        .await
        .unwrap();
    assert_eq!(
        problems,
        vec!["日语：读音标记 {朝|asa} 的读音只能有假名，不能有 a"]
    );
    let problems = update_line(&db, ids[1], Language::Zh, " ").await.unwrap();
    assert_eq!(problems, vec!["中文为空"]);
    assert_eq!(lines(&db, uuid).await[1].ja, "おはよう！");
}

#[tokio::test]
async fn the_voice_must_be_able_to_speak_the_language() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(&db, &script("二", &[("健二", "よう。", "哟。")]), 1).await;
    let id = lines(&db, uuid).await[0].id;

    let problems = update_line(&db, id, Language::Zh, "嗨。").await.unwrap();
    assert_eq!(problems, vec!["声音 健二 没有中文参考音"]);
    assert!(
        update_line(&db, id, Language::Ja, "やあ。")
            .await
            .unwrap()
            .is_empty()
    );

    delete_character(&db, "健二").await.unwrap();
    let problems = update_line(&db, id, Language::Ja, "おう。").await.unwrap();
    assert_eq!(problems, vec!["声音 健二 已经被删除"]);
    assert_eq!(lines(&db, uuid).await[0].ja, "やあ。");
}

#[tokio::test]
async fn changing_a_deleted_line_fails() {
    let db = memory_db().await;
    let (_, ids) = stored(&db).await;
    delete_lines(&db, &ids[..1]).await.unwrap();

    let error = update_line(&db, ids[0], Language::Zh, "早。")
        .await
        .unwrap_err();
    assert!(matches!(error, DramaError::LineNotFound));
}

#[tokio::test]
async fn deleting_lines_takes_their_audio() {
    let db = memory_db().await;
    let (uuid, ids) = stored(&db).await;
    save_audio(&db, ids[0], Language::Ja, b"ja").await.unwrap();

    assert_eq!(delete_lines(&db, &[ids[0], ids[2]]).await.unwrap(), 2);
    assert_eq!(delete_lines(&db, &[ids[0]]).await.unwrap(), 0);
    let left: Vec<LineId> = lines(&db, uuid).await.iter().map(|line| line.id).collect();
    assert_eq!(left, vec![ids[1]]);
    assert_eq!(audio(&db, ids[0], Language::Ja).await.unwrap(), None);
}

#[tokio::test]
async fn audio_is_saved_replaced_and_cleared() {
    let db = memory_db().await;
    let (_, ids) = stored(&db).await;

    save_audio(&db, ids[0], Language::Ja, b"first")
        .await
        .unwrap();
    save_audio(&db, ids[0], Language::Ja, b"second")
        .await
        .unwrap();
    save_audio(&db, ids[1], Language::Ja, b"ja").await.unwrap();
    save_audio(&db, ids[1], Language::Zh, b"zh").await.unwrap();
    assert_eq!(
        audio(&db, ids[0], Language::Ja).await.unwrap(),
        Some(b"second".to_vec())
    );
    assert_eq!(audio(&db, ids[0], Language::Zh).await.unwrap(), None);

    assert_eq!(clear_audio(&db, &ids, Language::Ja).await.unwrap(), 2);
    assert_eq!(audio(&db, ids[1], Language::Ja).await.unwrap(), None);
    assert_eq!(
        audio(&db, ids[1], Language::Zh).await.unwrap(),
        Some(b"zh".to_vec())
    );
}

#[tokio::test]
async fn saving_audio_needs_the_line_and_some_audio() {
    let db = memory_db().await;
    let (_, ids) = stored(&db).await;

    let empty = save_audio(&db, ids[0], Language::Ja, b"")
        .await
        .unwrap_err();
    assert_eq!(empty.to_string(), "音频为空");
    delete_lines(&db, &ids[..1]).await.unwrap();
    let gone = save_audio(&db, ids[0], Language::Ja, b"wav")
        .await
        .unwrap_err();
    assert!(matches!(gone, DramaError::LineNotFound));
}

#[tokio::test]
async fn missing_audio_lists_what_can_be_synthesized() {
    let db = memory_db().await;
    seeded_world(&db).await;
    let uuid = import(
        &db,
        &script(
            "二",
            &[
                ("旁白", "{春|はる}だ。", "春天了。"),
                ("健二", "よう。", "哟。"),
                ("美咲", "やあ。", "嗨。"),
            ],
        ),
        1,
    )
    .await;
    let ids: Vec<LineId> = lines(&db, uuid).await.iter().map(|line| line.id).collect();
    save_audio(&db, ids[0], Language::Zh, b"zh").await.unwrap();
    delete_character(&db, "美咲").await.unwrap();

    assert_eq!(
        missing_audio(&db, uuid).await.unwrap(),
        vec![
            SpeechItem {
                line: ids[0],
                language: Language::Ja,
                text: "はるだ。".into(),
                voice: "老陈".into(),
            },
            SpeechItem {
                line: ids[1],
                language: Language::Ja,
                text: "よう。".into(),
                voice: "健二".into(),
            },
        ]
    );
    let missing = missing_audio(&db, Uuid::from_u128(9)).await.unwrap_err();
    assert!(matches!(missing, DramaError::EpisodeNotFound));
}
