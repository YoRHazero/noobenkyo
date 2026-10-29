//! Probing the machines in the settings: one here, without ssh, run by the
//! system's python3.

use ai::{AiService, AiSettings, InvokerSettings, Machine, MachineProbe, MachineReport};

fn here(name: &str) -> Machine {
    Machine {
        name: name.into(),
        ssh: None,
        command_layers: vec![],
        runtimes: vec![],
        probe: None,
    }
}

#[tokio::test]
async fn keeps_what_the_last_probe_of_all_found() {
    let dir = tempfile::tempdir().unwrap();
    let service = AiService::open(&dir.path().join("ai")).unwrap();
    assert!(service.reports().is_empty());
    service
        .update_settings(AiSettings {
            invoker: InvokerSettings {
                machines: vec![here("a"), here("b")],
                ..InvokerSettings::default()
            },
            ..AiSettings::default()
        })
        .unwrap();

    let answered = MachineReport::Online {
        models_dir: None,
        qwen_tts: None,
        llama_cpp: None,
    };
    let reports = service.probe_all().await;
    let expected: Vec<MachineProbe> = ["a", "b"]
        .map(|machine| MachineProbe {
            machine: machine.into(),
            report: answered.clone(),
        })
        .into();
    assert_eq!(reports, expected);
    assert_eq!(service.reports(), expected);

    // A machine not in the settings is probed without being kept.
    assert_eq!(service.probe(&here("c")).await.unwrap(), answered);
    assert_eq!(service.reports(), expected);
}
