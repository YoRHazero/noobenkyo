//! The model page's commands: the ai settings read and replaced, and the
//! machines probed. The machine here has no ssh, so the system's python3
//! answers the probe.

use ai::{Machine, MachineReport};
use noobenkyo_core::commands::ai::probe::{
    ai_machine_reports, ai_probe_machine, ai_probe_machines,
};
use noobenkyo_core::commands::ai::settings::{
    ai_default_settings, ai_settings, ai_update_settings,
};
use noobenkyo_core::events::ai_machines_probed::AiMachinesProbed;
use tauri_specta::Event;
use tokio::sync::mpsc::unbounded_channel;

use crate::support::TestApp;

fn here(name: &str) -> Machine {
    Machine {
        name: name.into(),
        ssh: None,
        command_layers: vec![],
        runtimes: vec![],
        probe: None,
    }
}

const ANSWERED: MachineReport = MachineReport::Online {
    models_dir: None,
    qwen_tts: None,
    llama_cpp: None,
};

#[tokio::test]
async fn settings_are_replaced_whole_and_invalid_ones_refused() {
    let app = TestApp::fresh().await;
    assert_eq!(ai_settings(app.state()), ai_default_settings());

    let mut settings = ai_default_settings();
    settings.models.speak = Some("Qwen3-TTS-12Hz-1.7B-Base".into());
    ai_update_settings(app.state(), settings.clone()).unwrap();
    assert_eq!(ai_settings(app.state()), settings);

    let mut invalid = settings.clone();
    invalid.invoker.idle_timeout_secs = 0;
    let refused = ai_update_settings(app.state(), invalid).unwrap_err();
    assert_eq!(refused.message, "闲置超时至少 1 秒");
    assert_eq!(ai_settings(app.state()), settings);
}

#[tokio::test]
async fn probing_every_machine_keeps_the_reports_and_says_so() {
    let app = TestApp::fresh().await;
    let mut settings = ai_default_settings();
    settings.invoker.machines = vec![here("here")];
    ai_update_settings(app.state(), settings).unwrap();
    assert!(ai_machine_reports(app.state()).is_empty());
    let (sender, mut probed) = unbounded_channel();
    AiMachinesProbed::listen(&app.handle(), move |_| {
        let _ = sender.send(());
    });

    let reports = ai_probe_machines(app.handle()).await.unwrap();
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].machine, "here");
    assert_eq!(reports[0].report, ANSWERED);
    assert_eq!(ai_machine_reports(app.state()), reports);
    probed.recv().await.unwrap();

    // One machine as the form has it, not in the settings, and not kept.
    let report = ai_probe_machine(app.state(), here("draft")).await.unwrap();
    assert_eq!(report, ANSWERED);
    assert_eq!(ai_machine_reports(app.state()), reports);
}
