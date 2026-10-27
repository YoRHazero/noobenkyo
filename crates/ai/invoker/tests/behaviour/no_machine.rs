//! Calls that fail before any model runs; no GPU machine needed.

use ai_invoker::{
    InvokeError, Invoker, InvokerSettings, LlamaCppModel, Machine, QwenTtsModel, Runtime,
    TtsDesignRequest, TtsLanguage, TtsOp,
};
use serde_json::Map;

fn local(python: &str) -> InvokerSettings {
    InvokerSettings {
        machines: vec![Machine {
            name: "here".into(),
            ssh: None,
            command_layers: vec![],
            runtimes: vec![
                Runtime::LlamaCpp {
                    python: python.into(),
                    server: "/nowhere/llama-server".into(),
                    models: vec![LlamaCppModel {
                        name: "llm".into(),
                        path: "/nowhere/llm.gguf".into(),
                        args: vec![],
                    }],
                },
                Runtime::QwenTts {
                    python: python.into(),
                    models: vec![QwenTtsModel {
                        name: "base".into(),
                        op: TtsOp::TtsClone,
                        path: "/nowhere/base".into(),
                    }],
                },
            ],
        }],
        ..InvokerSettings::default()
    }
}

#[tokio::test]
async fn without_machines_nothing_runs() {
    let invoker = Invoker::new(InvokerSettings::default());
    let error = invoker.llm_chat("llm", Map::new(), |_| {}).await;
    assert_eq!(
        error,
        Err(InvokeError::NoMachine {
            model: "llm".into(),
            tried: vec![]
        })
    );
}

#[tokio::test]
async fn a_model_for_another_call_starts_nothing() {
    let invoker = Invoker::new(local("/nowhere/python3"));
    let request = TtsDesignRequest {
        text: "こんにちは".into(),
        language: TtsLanguage::Japanese,
        instruct: String::new(),
        seed: None,
    };
    let Err(InvokeError::NoMachine { tried, .. }) = invoker.tts_design("base", request).await
    else {
        panic!("expected NoMachine");
    };
    assert_eq!(
        tried,
        [(
            "here".into(),
            "base 只能做 tts_clone，不能做 tts_design".into()
        )]
    );
}

#[tokio::test]
async fn a_machine_that_fails_to_start_is_reported_with_its_reason() {
    let invoker = Invoker::new(local("/nowhere/python3"));
    let Err(InvokeError::NoMachine { model, tried }) =
        invoker.llm_chat("llm", Map::new(), |_| {}).await
    else {
        panic!("expected NoMachine");
    };
    assert_eq!(model, "llm");
    assert_eq!(tried.len(), 1);
    assert_eq!(tried[0].0, "here");
    assert!(
        tried[0].1.starts_with("启动 /nowhere/python3 失败"),
        "{}",
        tried[0].1
    );
}
