//! The inbox's commands, handed on as is: what the command line leaves is
//! listed, read, taken, written back, saved and discarded.

use ai::{InboxItem, InboxScript, ScriptTarget};
use noobenkyo_core::commands::dictionary::entries::{
    dictionary_default_import_options, dictionary_generated_import_options,
};
use noobenkyo_core::commands::inbox::entries::inbox_take_entries;
use noobenkyo_core::commands::inbox::list::inbox_list;
use noobenkyo_core::commands::inbox::scripts::{
    inbox_discard_script, inbox_read_script, inbox_replace_script, inbox_save_script,
};
use noobenkyo_core::commands::listening::episodes::listening_episode;

use crate::support::{SAMPLE_SCRIPT, TestApp, WORLD};

fn new_episode() -> ScriptTarget {
    ScriptTarget::NewEpisode {
        world: WORLD.into(),
    }
}

/// An inbox file holding a script for a new episode of 星见町.
fn script_file(text: &str) -> String {
    serde_json::json!({
        "kind": "script",
        "target": { "kind": "newEpisode", "world": WORLD },
        "text": text,
    })
    .to_string()
}

#[tokio::test]
async fn entries_left_in_the_data_directory_are_listed_and_taken() {
    let app = TestApp::fresh().await;
    let text = r#"{"writing": "諦める"}"#;
    app.deliver(
        "todo",
        &serde_json::json!({ "kind": "entries", "text": text }).to_string(),
    );

    assert_eq!(
        inbox_list(app.state()).await.unwrap(),
        [InboxItem::Entries {
            name: "todo".into()
        }]
    );
    assert_eq!(
        inbox_take_entries(app.state(), "todo".into())
            .await
            .unwrap(),
        text
    );
    assert!(inbox_list(app.state()).await.unwrap().is_empty());
}

#[tokio::test]
async fn a_script_is_edited_in_place_and_saved_as_an_episode() {
    let (app, _) = TestApp::with_world().await;
    app.deliver("ep", &script_file(SAMPLE_SCRIPT));
    assert_eq!(
        inbox_read_script(app.state(), "ep".into()).await.unwrap(),
        Some(InboxScript {
            target: new_episode(),
            text: SAMPLE_SCRIPT.into(),
        })
    );

    let edited = SAMPLE_SCRIPT.replace("海边的早晨", "灯塔之约");
    let problems = inbox_replace_script(app.state(), "ep".into(), new_episode(), edited.clone())
        .await
        .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    let rejected = inbox_replace_script(app.state(), "ep".into(), new_episode(), "{}".into())
        .await
        .unwrap();
    assert!(!rejected.is_empty());
    assert_eq!(
        inbox_read_script(app.state(), "ep".into())
            .await
            .unwrap()
            .unwrap()
            .text,
        edited
    );

    let episode = inbox_save_script(app.state(), "ep".into(), new_episode(), edited)
        .await
        .unwrap();
    let stored = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.title, "灯塔之约");
    assert_eq!(
        inbox_read_script(app.state(), "ep".into()).await.unwrap(),
        None
    );
}

#[tokio::test]
async fn a_discarded_script_leaves_the_inbox() {
    let app = TestApp::fresh().await;
    app.deliver("ep", &script_file(SAMPLE_SCRIPT));
    inbox_discard_script(app.state(), "ep".into())
        .await
        .unwrap();
    assert!(inbox_list(app.state()).await.unwrap().is_empty());
}

#[tokio::test]
async fn the_wrong_kind_is_refused_with_a_message() {
    let app = TestApp::fresh().await;
    app.deliver("ep", &script_file(SAMPLE_SCRIPT));
    let error = inbox_take_entries(app.state(), "ep".into())
        .await
        .unwrap_err();
    assert_eq!(error.message, "inbox 里的 ep 不是条目");
}

#[test]
fn delivered_entries_open_with_their_own_options() {
    assert_ne!(
        dictionary_generated_import_options(),
        dictionary_default_import_options()
    );
}
