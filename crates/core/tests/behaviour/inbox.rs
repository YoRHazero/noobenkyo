//! The inbox's commands: what is left in either source's folder is listed,
//! read, taken, written back after listening's check, saved into listening
//! and discarded.

use inbox::{InboxItem, InboxScript, InboxSource, ScriptTarget};
use noobenkyo_core::commands::dictionary::entries::{
    dictionary_default_import_options, dictionary_generated_import_options,
};
use noobenkyo_core::commands::inbox::entries::inbox_take_entries;
use noobenkyo_core::commands::inbox::list::inbox_list;
use noobenkyo_core::commands::inbox::scripts::{
    inbox_discard_script, inbox_read_script, inbox_replace_script, inbox_save_script,
};
use noobenkyo_core::commands::listening::episodes::{listening_episode, listening_episodes};

use crate::support::{SAMPLE_SCRIPT, TestApp, WORLD};

const AGENT: InboxSource = InboxSource::Agent;

fn new_episode() -> ScriptTarget {
    ScriptTarget::NewEpisode {
        world: WORLD.into(),
    }
}

/// An inbox file holding a script for `target`.
fn script_file(target: &ScriptTarget, text: &str) -> String {
    serde_json::json!({
        "kind": "script",
        "target": target,
        "text": text,
    })
    .to_string()
}

/// The sample script with another title, as an edit in the preview makes it.
fn retitled(title: &str) -> String {
    SAMPLE_SCRIPT.replace("海边的早晨", title)
}

#[tokio::test]
async fn entries_left_in_the_data_directory_are_listed_and_taken() {
    let app = TestApp::fresh().await;
    let text = r#"{"writing": "諦める"}"#;
    app.deliver(
        AGENT,
        "todo",
        &serde_json::json!({ "kind": "entries", "text": text }).to_string(),
    );

    assert_eq!(
        inbox_list(app.state()).await.unwrap(),
        [InboxItem::Entries {
            source: AGENT,
            name: "todo".into()
        }]
    );
    assert_eq!(
        inbox_take_entries(app.state(), AGENT, "todo".into())
            .await
            .unwrap(),
        text
    );
    assert!(inbox_list(app.state()).await.unwrap().is_empty());
}

#[tokio::test]
async fn a_script_from_the_app_s_own_models_is_listed_and_read() {
    let app = TestApp::fresh().await;
    app.deliver(
        InboxSource::Ai,
        "ep",
        &script_file(&new_episode(), SAMPLE_SCRIPT),
    );

    assert_eq!(
        inbox_list(app.state()).await.unwrap(),
        [InboxItem::Script {
            source: InboxSource::Ai,
            name: "ep".into(),
            target: new_episode(),
        }]
    );
    assert_eq!(
        inbox_read_script(app.state(), InboxSource::Ai, "ep".into())
            .await
            .unwrap(),
        Some(InboxScript {
            target: new_episode(),
            text: SAMPLE_SCRIPT.into(),
        })
    );
    assert_eq!(
        inbox_read_script(app.state(), AGENT, "ep".into())
            .await
            .unwrap(),
        None
    );
}

#[tokio::test]
async fn a_script_is_edited_in_place_and_saved_as_an_episode() {
    let (app, _) = TestApp::with_world().await;
    app.deliver(AGENT, "ep", &script_file(&new_episode(), SAMPLE_SCRIPT));
    assert_eq!(
        inbox_read_script(app.state(), AGENT, "ep".into())
            .await
            .unwrap(),
        Some(InboxScript {
            target: new_episode(),
            text: SAMPLE_SCRIPT.into(),
        })
    );

    let edited = retitled("灯塔之约");
    let problems = inbox_replace_script(
        app.state(),
        AGENT,
        "ep".into(),
        new_episode(),
        edited.clone(),
    )
    .await
    .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    let rejected =
        inbox_replace_script(app.state(), AGENT, "ep".into(), new_episode(), "{}".into())
            .await
            .unwrap();
    assert!(!rejected.is_empty());
    assert_eq!(
        inbox_read_script(app.state(), AGENT, "ep".into())
            .await
            .unwrap()
            .unwrap()
            .text,
        edited
    );

    let episode = inbox_save_script(app.state(), AGENT, "ep".into(), new_episode(), edited)
        .await
        .unwrap();
    let stored = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.title, "灯塔之约");
    assert_eq!(
        inbox_read_script(app.state(), AGENT, "ep".into())
            .await
            .unwrap(),
        None
    );
}

#[tokio::test]
async fn an_edit_is_checked_against_where_it_goes() {
    let (app, episode) = TestApp::with_world().await;
    let target = ScriptTarget::Append { episode };
    app.deliver(AGENT, "more", &script_file(&target, SAMPLE_SCRIPT));

    let problems = inbox_replace_script(
        app.state(),
        AGENT,
        "more".into(),
        ScriptTarget::NewEpisode {
            world: "火星".into(),
        },
        SAMPLE_SCRIPT.into(),
    )
    .await
    .unwrap();
    assert_eq!(problems, ["世界 火星 不存在"]);
    assert_eq!(
        inbox_read_script(app.state(), AGENT, "more".into())
            .await
            .unwrap()
            .unwrap()
            .target,
        target
    );
}

#[tokio::test]
async fn writing_back_a_script_no_longer_waiting_says_so_before_any_check() {
    let (app, _) = TestApp::with_world().await;
    let error = inbox_replace_script(app.state(), AGENT, "ep".into(), new_episode(), "{}".into())
        .await
        .unwrap_err();
    assert_eq!(error.message, "inbox 里没有 ep");
}

#[tokio::test]
async fn saving_a_continuation_appends_and_returns_the_episode() {
    let (app, episode) = TestApp::with_world().await;
    let target = ScriptTarget::Append { episode };
    app.deliver(AGENT, "more", &script_file(&target, SAMPLE_SCRIPT));

    let saved = inbox_save_script(
        app.state(),
        AGENT,
        "more".into(),
        target,
        SAMPLE_SCRIPT.into(),
    )
    .await
    .unwrap();
    assert_eq!(saved, episode);
    let stored = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.lines.len(), 16);
    assert!(inbox_list(app.state()).await.unwrap().is_empty());
}

#[tokio::test]
async fn a_script_no_longer_waiting_is_not_stored_again() {
    let (app, _) = TestApp::with_world().await;
    let error = inbox_save_script(
        app.state(),
        AGENT,
        "ep".into(),
        new_episode(),
        SAMPLE_SCRIPT.into(),
    )
    .await
    .unwrap_err();
    assert_eq!(error.message, "inbox 里没有 ep");
    assert_eq!(
        listening_episodes(app.state(), WORLD.into())
            .await
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn a_script_that_cannot_be_stored_stays_in_the_inbox() {
    let (app, _) = TestApp::with_world().await;
    app.deliver(AGENT, "ep", &script_file(&new_episode(), SAMPLE_SCRIPT));

    let error = inbox_save_script(app.state(), AGENT, "ep".into(), new_episode(), retitled(""))
        .await
        .unwrap_err();
    assert!(error.message.starts_with("剧本不通过"), "{}", error.message);
    assert!(
        inbox_read_script(app.state(), AGENT, "ep".into())
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn a_discarded_script_leaves_the_inbox() {
    let app = TestApp::fresh().await;
    app.deliver(AGENT, "ep", &script_file(&new_episode(), SAMPLE_SCRIPT));
    inbox_discard_script(app.state(), AGENT, "ep".into())
        .await
        .unwrap();
    assert!(inbox_list(app.state()).await.unwrap().is_empty());
}

#[tokio::test]
async fn the_wrong_kind_is_refused_with_a_message() {
    let app = TestApp::fresh().await;
    app.deliver(AGENT, "ep", &script_file(&new_episode(), SAMPLE_SCRIPT));
    let error = inbox_take_entries(app.state(), AGENT, "ep".into())
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
