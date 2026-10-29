//! Probing a machine: here, without ssh, on a made-up models directory, run
//! by the system's python3; and a machine that cannot be reached.

use std::path::Path;
use std::time::Duration;

use ai_invoker::{InvokerSettingsError, Machine, MachineReport, ProbeDirs, Runtime, TtsOp, probe};

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

fn tts_config(kind: &str) -> String {
    format!(r#"{{ "model_type": "qwen3_tts", "tts_model_type": "{kind}" }}"#)
}

fn machine(ssh: Option<&str>, probe: Option<ProbeDirs>) -> Machine {
    Machine {
        name: "here".into(),
        ssh: ssh.map(str::to_owned),
        command_layers: vec![],
        runtimes: vec![],
        probe,
    }
}

fn path(root: &Path, relative: &str) -> String {
    root.join(relative).to_str().unwrap().to_owned()
}

#[tokio::test]
async fn finds_the_models_and_says_what_is_missing() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path();
    write(&root.join("models/Base/config.json"), &tts_config("base"));
    write(
        &root.join("models/Design/config.json"),
        &tts_config("voice_design"),
    );
    write(
        &root.join("models/Custom/config.json"),
        &tts_config("custom_voice"),
    );
    write(
        &root.join("models/Whisper/config.json"),
        r#"{ "model_type": "whisper" }"#,
    );
    // Not one level down, so not a qwen-tts model.
    write(
        &root.join("models/deep/Base/config.json"),
        &tts_config("base"),
    );
    write(&root.join("models/Small-GGUF/Small-Q8_0.gguf"), "");
    write(&root.join("models/.cache/Hidden.gguf"), "");
    write(&root.join("llama.cpp/bin/llama-server"), "");
    let dirs = ProbeDirs {
        models: path(root, "models"),
        // No .venv in it, so the probe falls back to the system's python.
        qwen_tts: Some(path(root, "qwen-tts")),
        llama_cpp: Some(path(root, "llama.cpp")),
        llama_python: None,
    };

    let report = probe(&machine(None, Some(dirs)), Duration::from_secs(5))
        .await
        .unwrap();

    let MachineReport::Online {
        models_dir: Some(true),
        qwen_tts: Some(qwen_tts),
        llama_cpp: Some(llama_cpp),
    } = report
    else {
        panic!("{report:?}");
    };
    let qwen_python = path(root, "qwen-tts/.venv/bin/python");
    assert_eq!(qwen_tts.problem, Some(format!("找不到 {qwen_python}")));
    let Runtime::QwenTts { models, .. } = qwen_tts.runtime else {
        panic!("qwen-tts");
    };
    let found: Vec<(&str, TtsOp)> = models.iter().map(|m| (m.name.as_str(), m.op)).collect();
    assert_eq!(
        found,
        [("Base", TtsOp::TtsClone), ("Design", TtsOp::TtsDesign)]
    );
    assert_eq!(models[0].path, path(root, "models/Base"));
    // llama.cpp runs on qwen-tts's python, which is missing; its server is there.
    assert_eq!(llama_cpp.problem, Some(format!("找不到 {qwen_python}")));
    let Runtime::LlamaCpp { server, models, .. } = llama_cpp.runtime else {
        panic!("llama.cpp");
    };
    assert_eq!(server, path(root, "llama.cpp/bin/llama-server"));
    let names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, ["Small-Q8_0"]);
}

#[tokio::test]
async fn without_directories_it_only_sees_the_machine_answer() {
    let report = probe(&machine(None, None), Duration::from_secs(5))
        .await
        .unwrap();
    assert_eq!(
        report,
        MachineReport::Online {
            models_dir: None,
            qwen_tts: None,
            llama_cpp: None,
        }
    );
}

#[tokio::test]
async fn a_machine_that_cannot_be_reached_is_offline() {
    // TEST-NET-1: nothing answers there.
    let report = probe(
        &machine(Some("nobody@192.0.2.1"), None),
        Duration::from_secs(1),
    )
    .await
    .unwrap();
    let MachineReport::Offline { reason } = report else {
        panic!("{report:?}");
    };
    assert!(reason.starts_with("连接失败"), "{reason}");
}

#[tokio::test]
async fn a_machine_that_would_be_refused_is_not_probed() {
    let dirs = ProbeDirs {
        models: "/models".into(),
        qwen_tts: Some("/opt/qwen-tts; rm -rf ~".into()),
        ..ProbeDirs::default()
    };
    let refused = probe(&machine(None, Some(dirs)), Duration::from_secs(1)).await;
    assert_eq!(
        refused,
        Err(InvokerSettingsError::UnsafeOnCommandLine {
            machine: "here".into(),
            value: "/opt/qwen-tts; rm -rf ~".into(),
        })
    );
}
