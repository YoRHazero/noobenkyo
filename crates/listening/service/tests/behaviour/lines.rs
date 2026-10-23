use listening::{Language, LineId, ListeningError};

use crate::support::{sample_episode, seeded, service};

#[tokio::test]
async fn synthesizing_an_episode() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let uuid = sample_episode(&listening).await;

    let missing = listening.missing_audio(uuid).await.unwrap();
    // 8 Japanese recordings, and Chinese for every line but 健二's two.
    assert_eq!(missing.len(), 8 + 6);
    assert_eq!(missing[0].text, "しおかぜがふく、はるのあさ。");
    assert_eq!(missing[0].voice, "老陈");
    for item in &missing {
        listening
            .save_audio(item.line, item.language, b"wav")
            .await
            .unwrap();
    }

    assert!(listening.missing_audio(uuid).await.unwrap().is_empty());
    let summary = &listening.episodes("星见町").await.unwrap()[0];
    assert_eq!((summary.missing_ja, summary.missing_zh), (0, 2));
    assert_eq!(
        listening
            .audio(missing[0].line, Language::Ja)
            .await
            .unwrap(),
        Some(b"wav".to_vec())
    );
}

#[tokio::test]
async fn editing_and_resynthesizing_lines() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let uuid = sample_episode(&listening).await;
    let ids: Vec<LineId> = listening
        .episode(uuid)
        .await
        .unwrap()
        .unwrap()
        .lines
        .iter()
        .map(|line| line.id)
        .collect();
    for id in &ids[..2] {
        for language in Language::ALL {
            listening.save_audio(*id, language, b"wav").await.unwrap();
        }
    }

    let problems = listening
        .update_line(ids[0], Language::Zh, "海风轻拂的春日早晨。")
        .await
        .unwrap();
    assert!(problems.is_empty());
    assert_eq!(listening.audio(ids[0], Language::Zh).await.unwrap(), None);
    assert!(
        listening
            .audio(ids[0], Language::Ja)
            .await
            .unwrap()
            .is_some()
    );

    assert_eq!(listening.clear_audio(&ids, Language::Ja).await.unwrap(), 2);
    assert_eq!(listening.delete_lines(&ids[1..3]).await.unwrap(), 2);
    let error = listening
        .save_audio(ids[1], Language::Ja, b"wav")
        .await
        .unwrap_err();
    assert!(matches!(error, ListeningError::Drama(_)));
    assert_eq!(error.to_string(), "这一行不存在");
    assert_eq!(
        listening.episode(uuid).await.unwrap().unwrap().lines.len(),
        6
    );
}
