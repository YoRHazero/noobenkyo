//! Listening's commands, handed on as is. Audio crosses as plain bytes.

use listening::{Language, Reference, Role, RubySegment, ScriptPreview, WorldSummary};
use noobenkyo_core::commands::listening::characters::{
    listening_add_reference, listening_character, listening_character_usage, listening_characters,
    listening_create_character, listening_delete_character, listening_reference,
};
use noobenkyo_core::commands::listening::episodes::{
    listening_delete_episode, listening_episode, listening_episodes, listening_update_episode,
};
use noobenkyo_core::commands::listening::lines::{
    listening_audio, listening_clear_audio, listening_delete_lines, listening_update_line,
};
use noobenkyo_core::commands::listening::script::{
    listening_append_script, listening_import_script, listening_preview_script,
};
use noobenkyo_core::commands::listening::worlds::{
    listening_add_cast, listening_create_world, listening_delete_world, listening_remove_cast,
    listening_set_role, listening_update_setting, listening_world, listening_worlds,
};

use crate::support::{
    SAMPLE_SCRIPT, TestApp, WORLD, main_role, new_character_with, reference_audio,
};

/// The command line for agents looks for listening.db in the same place.
#[tokio::test]
async fn the_first_start_creates_the_listening_data() {
    let app = TestApp::fresh().await;
    assert!(app.data_dir().join("listening/listening.db").is_file());
    assert!(listening_worlds(app.state()).await.unwrap().is_empty());
    assert!(listening_characters(app.state()).await.unwrap().is_empty());
}

#[tokio::test]
async fn worlds_and_their_casts() {
    let (app, _) = TestApp::with_world().await;
    assert_eq!(
        listening_worlds(app.state()).await.unwrap(),
        vec![WorldSummary {
            name: WORLD.into(),
            episodes: 1
        }]
    );

    listening_update_setting(app.state(), WORLD.into(), "下雨的小镇。".into())
        .await
        .unwrap();
    listening_set_role(
        app.state(),
        WORLD.into(),
        "店员".into(),
        main_role("便利店的店员。"),
    )
    .await
    .unwrap();
    assert!(
        listening_remove_cast(app.state(), WORLD.into(), "健二".into())
            .await
            .unwrap()
    );
    let world = listening_world(app.state(), WORLD.into())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(world.setting, "下雨的小镇。");
    let cast: Vec<(&str, &Role)> = world
        .cast
        .iter()
        .map(|member| (member.character.as_str(), &member.role))
        .collect();
    assert_eq!(
        cast,
        vec![
            ("老陈", &Role::Narrator),
            ("美咲", &main_role("高中生，喜欢拍照。")),
            ("店员", &main_role("便利店的店员。")),
        ]
    );

    listening_add_cast(app.state(), WORLD.into(), "健二".into(), Role::Minor)
        .await
        .unwrap();
    assert!(
        listening_delete_world(app.state(), WORLD.into())
            .await
            .unwrap()
    );
    assert!(listening_worlds(app.state()).await.unwrap().is_empty());
}

#[tokio::test]
async fn deleting_a_character_takes_it_out_of_casts() {
    let (app, _) = TestApp::with_world().await;
    assert_eq!(listening_characters(app.state()).await.unwrap().len(), 4);
    assert_eq!(
        listening_character_usage(app.state(), "店员".into())
            .await
            .unwrap(),
        vec![WORLD]
    );
    assert!(
        listening_delete_character(app.state(), "店员".into())
            .await
            .unwrap()
    );
    assert!(
        listening_character(app.state(), "店员".into())
            .await
            .unwrap()
            .is_none()
    );
    let world = listening_world(app.state(), WORLD.into())
        .await
        .unwrap()
        .unwrap();
    assert!(world.cast.iter().all(|member| member.character != "店员"));
}

#[tokio::test]
async fn episodes_and_their_lines() {
    let (app, episode) = TestApp::with_world().await;
    let listed = listening_episodes(app.state(), WORLD.into()).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(
        (listed[0].uuid, listed[0].number, listed[0].lines),
        (episode, 1.0, 8)
    );

    let problems = listening_update_episode(
        app.state(),
        episode,
        2.5,
        "下雨的早晨".into(),
        "两人在便利店躲雨。".into(),
    )
    .await
    .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    let detail = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap();
    assert_eq!((detail.number, detail.title.as_str()), (2.5, "下雨的早晨"));

    let ids: Vec<_> = detail.lines.iter().map(|line| line.id).collect();
    let problems = listening_update_line(app.state(), ids[0], Language::Zh, "下雨的早晨。".into())
        .await
        .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(
        listening_delete_lines(app.state(), ids[1..3].to_vec())
            .await
            .unwrap(),
        2
    );
    let lines = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap()
        .lines;
    assert_eq!(lines.len(), 6);
    assert_eq!(lines[0].zh, "下雨的早晨。");

    assert!(
        listening_delete_episode(app.state(), episode)
            .await
            .unwrap()
    );
    assert!(
        listening_episode(app.state(), episode)
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn listening_errors_reach_the_frontend_in_its_own_words() {
    let (app, _) = TestApp::with_world().await;
    let error = listening_create_world(app.state(), WORLD.into(), String::new())
        .await
        .unwrap_err();
    assert_eq!(error.message, "世界 星见町 已经存在");
}

#[tokio::test]
async fn characters_are_created_with_the_references_uploaded() {
    let app = TestApp::fresh().await;
    let problems = listening_create_character(
        app.state(),
        new_character_with("美咲", "みさき", &[Language::Ja]),
    )
    .await
    .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    assert_eq!(
        listening_reference(app.state(), "美咲".into(), Language::Ja)
            .await
            .unwrap(),
        Some(Reference {
            audio: reference_audio(Language::Ja),
            text: "参考。".into()
        })
    );
    assert!(
        listening_reference(app.state(), "美咲".into(), Language::Zh)
            .await
            .unwrap()
            .is_none()
    );

    listening_add_reference(
        app.state(),
        "美咲".into(),
        Language::Zh,
        reference_audio(Language::Zh),
        "参考。".into(),
    )
    .await
    .unwrap();
    let added = listening_reference(app.state(), "美咲".into(), Language::Zh)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(added.audio, reference_audio(Language::Zh));
    let error = listening_add_reference(
        app.state(),
        "美咲".into(),
        Language::Zh,
        reference_audio(Language::Zh),
        "参考。".into(),
    )
    .await
    .unwrap_err();
    assert_eq!(error.message, "角色 美咲 已经有中文参考音");

    // References can come later: only synthesis needs them.
    let problems =
        listening_create_character(app.state(), new_character_with("健二", "けんじ", &[]))
            .await
            .unwrap();
    assert!(problems.is_empty(), "{problems:?}");
    let created = listening_character(app.state(), "健二".into())
        .await
        .unwrap()
        .unwrap();
    assert!(created.references.is_empty());
}

/// Synthesis goes through the ai group, so the audio is saved through the
/// service here.
#[tokio::test]
async fn a_line_plays_the_audio_saved_for_it() {
    let (app, episode) = TestApp::with_world().await;
    let line = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap()
        .lines[0]
        .id;
    assert!(
        listening_audio(app.state(), line, Language::Ja)
            .await
            .unwrap()
            .is_none()
    );
    app.state()
        .listening
        .save_audio(line, Language::Ja, b"RIFF line")
        .await
        .unwrap();
    assert_eq!(
        listening_audio(app.state(), line, Language::Ja)
            .await
            .unwrap(),
        Some(b"RIFF line".to_vec())
    );
    assert!(
        listening_audio(app.state(), line, Language::Zh)
            .await
            .unwrap()
            .is_none()
    );

    assert_eq!(
        listening_clear_audio(app.state(), vec![line], Language::Ja)
            .await
            .unwrap(),
        1
    );
    assert!(
        listening_audio(app.state(), line, Language::Ja)
            .await
            .unwrap()
            .is_none()
    );
}

/// What the preview shows is what storing the script keeps.
#[tokio::test]
async fn the_preview_shows_the_script_as_it_would_be_stored() {
    let (app, episode) = TestApp::with_world().await;
    let ScriptPreview::Passed {
        title,
        summary,
        lines,
    } = listening_preview_script(SAMPLE_SCRIPT.to_owned()).await
    else {
        panic!("the sample script passes");
    };
    let stored = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap();
    assert_eq!((title, summary), (stored.title, stored.summary));
    let previewed: Vec<_> = lines
        .iter()
        .map(|line| (&line.speaker, &line.ja, &line.ja_ruby, &line.zh))
        .collect();
    let kept: Vec<_> = stored
        .lines
        .iter()
        .map(|line| (&line.speaker, &line.ja, &line.ja_ruby, &line.zh))
        .collect();
    assert_eq!(previewed, kept);
    assert_eq!(
        lines[0].ja_ruby[0],
        RubySegment::Ruby {
            writing: "潮風".into(),
            reading: "しおかぜ".into()
        }
    );

    assert_eq!(
        listening_preview_script("{}".to_owned()).await,
        ScriptPreview::Failed {
            problems: vec![
                "缺少 title".into(),
                "缺少 summary".into(),
                "缺少 lines".into()
            ]
        }
    );
}

/// One more line for the sample script, with a summary of the whole episode.
const CONTINUATION: &str = r#"{
  "title": "不用",
  "summary": "两人放学后一起去看了灯塔。",
  "lines": [{ "speaker": "美咲", "ja": "{灯台|とうだい}、きれいだね。", "zh": "灯塔真美啊。" }]
}"#;

#[tokio::test]
async fn scripts_are_stored_and_continued() {
    let (app, episode) = TestApp::with_world().await;
    listening_append_script(app.state(), episode, CONTINUATION.into())
        .await
        .unwrap();
    let detail = listening_episode(app.state(), episode)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        (detail.title.as_str(), detail.summary.as_str()),
        ("海边的早晨", "两人放学后一起去看了灯塔。")
    );
    assert_eq!(detail.lines.len(), 9);
    assert_eq!(detail.lines[8].voice, "美咲");

    let second = listening_import_script(app.state(), WORLD.into(), SAMPLE_SCRIPT.into())
        .await
        .unwrap();
    let listed: Vec<_> = listening_episodes(app.state(), WORLD.into())
        .await
        .unwrap()
        .iter()
        .map(|summary| (summary.uuid, summary.number))
        .collect();
    assert_eq!(listed, vec![(episode, 1.0), (second, 2.0)]);
}

#[tokio::test]
async fn scripts_that_cannot_be_stored_say_why() {
    let (app, episode) = TestApp::with_world().await;
    let error = listening_import_script(app.state(), WORLD.into(), "{}".into())
        .await
        .unwrap_err();
    assert_eq!(
        error.message,
        "剧本不通过：缺少 title；缺少 summary；缺少 lines"
    );
    let error = listening_append_script(app.state(), episode, "[]".into())
        .await
        .unwrap_err();
    assert!(
        error.message.starts_with("剧本不通过："),
        "{}",
        error.message
    );

    listening_create_world(app.state(), "空镇".into(), String::new())
        .await
        .unwrap();
    let error = listening_import_script(app.state(), "空镇".into(), SAMPLE_SCRIPT.into())
        .await
        .unwrap_err();
    assert_eq!(error.message, "世界 空镇 缺少旁白、主要角色");
}
