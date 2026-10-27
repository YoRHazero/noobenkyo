//! Executors starting, failing to start, idling out, dying and closing.

use std::time::{Duration, Instant};

use ai_invoker::{InvokeError, Runtime};
use tokio::sync::mpsc;

use crate::support::{
    LLM, TTS_CLONE, chat, gpu_invoker, gpu_memory, kill_executor, logged, wait_for_empty_gpu,
};

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn startup_failures_name_the_reason() {
    let (invoker, _) = gpu_invoker(|settings| {
        for runtime in &mut settings.machines[0].runtimes {
            match runtime {
                Runtime::LlamaCpp { models, .. } => models[0].path = "/nowhere/model.gguf".into(),
                // The system python has none of qwen-tts's libraries.
                Runtime::QwenTts { python, .. } => *python = "/usr/bin/python3".into(),
            }
        }
    });
    let llm = invoker.llm_chat(LLM, chat("はい", 4), |_| {}).await;
    eprintln!("{}", llm.as_ref().unwrap_err());
    let Err(InvokeError::NoMachine { tried, .. }) = llm else {
        panic!("{llm:?}")
    };
    assert_eq!(tried[0].1, "model path not found: /nowhere/model.gguf");

    let request = ai_invoker::TtsCloneRequest {
        text: "はい".into(),
        language: ai_invoker::TtsLanguage::Japanese,
        ref_audio: vec![],
        ref_text: String::new(),
    };
    let tts = invoker.tts_clone(TTS_CLONE, request).await;
    eprintln!("{}", tts.as_ref().unwrap_err());
    let Err(InvokeError::NoMachine { tried, .. }) = tts else {
        panic!("{tts:?}")
    };
    assert!(
        tried[0].1.starts_with("ModuleNotFoundError"),
        "{}",
        tried[0].1
    );
}

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn an_idle_executor_closes_and_a_new_one_starts_when_needed() {
    let (invoker, machine) = gpu_invoker(|settings| settings.idle_timeout_secs = 3);
    invoker
        .llm_chat(LLM, chat("「はい」とだけ答えてください。", 16), |_| {})
        .await
        .unwrap();
    let finished = Instant::now();
    wait_for_empty_gpu(&machine).await;
    eprintln!(
        "GPU empty {:.1} s after the call with a 3 s idle timeout",
        finished.elapsed().as_secs_f64()
    );
    assert_eq!(logged("闲置超时").len(), 1);
    invoker
        .llm_chat(LLM, chat("「はい」とだけ答えてください。", 16), |_| {})
        .await
        .unwrap();
    assert_eq!(logged(&format!("{LLM} started")).len(), 2);
    drop(invoker);
    wait_for_empty_gpu(&machine).await;
}

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn a_killed_executor_fails_its_call_and_is_replaced() {
    let (invoker, machine) = gpu_invoker(|_| {});
    let (tokens, mut written) = mpsc::unbounded_channel();
    let story = chat("日本語で長い物語を書いてください。", 2000);
    let call = invoker.llm_chat(LLM, story, |p| {
        let _ = tokens.send(p.tokens);
    });
    let kill = async {
        written.recv().await;
        let sent = Instant::now();
        kill_executor(&machine).await;
        sent
    };
    let ((result, failed), sent) = tokio::join!(async { (call.await, Instant::now()) }, kill);
    eprintln!(
        "call failed {:.2} s after sending the kill: {}",
        failed.duration_since(sent).as_secs_f64(),
        result.as_ref().unwrap_err()
    );
    assert!(matches!(result, Err(InvokeError::ExecutorFailed { .. })));
    // llama-server goes with it.
    wait_for_empty_gpu(&machine).await;
    invoker
        .llm_chat(LLM, chat("「はい」とだけ答えてください。", 16), |_| {})
        .await
        .unwrap();
    assert_eq!(logged(&format!("{LLM} started")).len(), 2);
    drop(invoker);
    wait_for_empty_gpu(&machine).await;
}

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn dropping_the_invoker_mid_call_frees_the_gpu() {
    let (invoker, machine) = gpu_invoker(|_| {});
    let (tokens, mut written) = mpsc::unbounded_channel();
    tokio::select! {
        reply = invoker.llm_chat(LLM, chat("日本語で長い物語を書いてください。", 2000), |p| { let _ = tokens.send(p.tokens); }) => {
            panic!("finished early: {reply:?}")
        }
        _ = written.recv() => {}
    }
    eprintln!(
        "GPU memory while writing: {} MiB",
        gpu_memory(&machine).await
    );
    drop(invoker);
    let waited = wait_for_empty_gpu(&machine).await;
    assert!(waited < Duration::from_secs(15));
}
