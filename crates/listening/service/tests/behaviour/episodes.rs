use listening::Uuid;

use crate::support::{SAMPLE_SCRIPT, sample_episode, seeded, service};

#[tokio::test]
async fn episodes_are_numbered_and_edited() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let first = sample_episode(&listening).await;
    let second = sample_episode(&listening).await;

    let problems = listening
        .update_episode(first, 2.5, "海边的早晨（上）", "上半集。")
        .await
        .unwrap();
    assert!(problems.is_empty());
    let listed: Vec<(Uuid, f64)> = listening
        .episodes("星见町")
        .await
        .unwrap()
        .iter()
        .map(|episode| (episode.uuid, episode.number))
        .collect();
    assert_eq!(listed, vec![(second, 2.0), (first, 2.5)]);
    let third = listening
        .import_script("星见町", SAMPLE_SCRIPT)
        .await
        .unwrap();
    assert_eq!(listening.episode(third).await.unwrap().unwrap().number, 3.0);

    let problems = listening
        .update_episode(first, 1.0, " ", "上半集。")
        .await
        .unwrap();
    assert_eq!(problems, vec!["标题为空"]);
}

#[tokio::test]
async fn deleting_an_episode() {
    let dir = tempfile::tempdir().unwrap();
    let listening = service(&dir).await;
    seeded(&listening).await;
    let uuid = sample_episode(&listening).await;

    assert!(listening.delete_episode(uuid).await.unwrap());
    assert!(!listening.delete_episode(uuid).await.unwrap());
    assert_eq!(listening.worlds().await.unwrap()[0].episodes, 0);
}
