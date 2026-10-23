use listening::Language;

use crate::support::{sample_episode, seeded, service};

#[tokio::test]
async fn everything_is_there_after_reopening() {
    let dir = tempfile::tempdir().unwrap();
    let uuid = {
        let listening = service(&dir).await;
        seeded(&listening).await;
        let uuid = sample_episode(&listening).await;
        let first = listening.episode(uuid).await.unwrap().unwrap().lines[0].id;
        listening
            .save_audio(first, Language::Ja, b"wav")
            .await
            .unwrap();
        uuid
    };

    let listening = service(&dir).await;
    assert_eq!(listening.characters().await.unwrap().len(), 4);
    assert_eq!(
        listening.world("星见町").await.unwrap().unwrap().cast.len(),
        4
    );
    let detail = listening.episode(uuid).await.unwrap().unwrap();
    assert_eq!(detail.lines.len(), 8);
    assert_eq!(detail.lines[0].audio, vec![Language::Ja]);
}
