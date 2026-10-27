//! llm_chat on the GPU machine.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use ai_invoker::LlmFinish;
use tokio::sync::mpsc;

use crate::support::{LLM, chat, gpu_invoker, gpu_memory, logged, wait_for_empty_gpu};

const STORY: &str = "日本語で、海辺の町に住む猫の長い物語を書いてください。";

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn chat_reports_progress_and_keeps_the_executor_for_the_next_call() {
    let (invoker, machine) = gpu_invoker(|_| {});
    let progress = Mutex::new(Vec::new());
    let start = Instant::now();
    let reply = invoker
        .llm_chat(LLM, chat(STORY, 200), |p| {
            progress.lock().unwrap().push(p.tokens)
        })
        .await
        .unwrap();
    let first = start.elapsed();
    let progress = progress.into_inner().unwrap();
    eprintln!(
        "first call (start included) {:.1} s, {} tokens, {:?}, progress {progress:?}",
        first.as_secs_f64(),
        reply.tokens,
        reply.finish
    );
    assert_eq!(reply.finish, LlmFinish::Length);
    assert_eq!(reply.tokens, 200);
    assert_eq!(reply.machine, machine.name);
    assert!(progress.starts_with(&[64, 128]), "{progress:?}");
    assert!(
        progress
            .iter()
            .all(|tokens| tokens % 64 == 0 && *tokens <= 200)
    );
    eprintln!(
        "GPU memory with {LLM} loaded: {} MiB",
        gpu_memory(&machine).await
    );

    let start = Instant::now();
    let reply = invoker
        .llm_chat(LLM, chat("「はい」とだけ答えてください。", 16), |_| {})
        .await
        .unwrap();
    let second = start.elapsed();
    eprintln!(
        "second call {:.2} s: {:?}",
        second.as_secs_f64(),
        reply.text
    );
    assert_eq!(reply.finish, LlmFinish::Stop);
    assert_eq!(logged(&format!("{LLM} started")).len(), 1);
    drop(invoker);
    wait_for_empty_gpu(&machine).await;
}

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn dropping_a_chat_stops_it_and_the_next_call_runs_right_after() {
    let (invoker, machine) = gpu_invoker(|_| {});
    invoker
        .llm_chat(LLM, chat("「はい」とだけ答えてください。", 16), |_| {})
        .await
        .unwrap();
    let (tokens, mut written) = mpsc::unbounded_channel();
    tokio::select! {
        reply = invoker.llm_chat(LLM, chat(STORY, 2000), |p| { let _ = tokens.send(p.tokens); }) => {
            panic!("finished before the cancel: {reply:?}")
        }
        _ = written.recv() => {}
    }
    let cancelled = Instant::now();
    let reply = invoker
        .llm_chat(LLM, chat("「はい」とだけ答えてください。", 16), |_| {})
        .await
        .unwrap();
    eprintln!(
        "next call done {:.2} s after the cancel: {:?}",
        cancelled.elapsed().as_secs_f64(),
        reply.text
    );
    let stopped = logged("after its cancel");
    assert_eq!(stopped.len(), 1, "{stopped:?}");
    assert!(cancelled.elapsed() < Duration::from_secs(5));
    drop(invoker);
    wait_for_empty_gpu(&machine).await;
}

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn calls_wait_for_the_busy_machine_and_share_its_executor() {
    let (invoker, machine) = gpu_invoker(|_| {});
    let start = Instant::now();
    let ask = |n: u32| {
        let invoker = &invoker;
        async move {
            let reply = invoker
                .llm_chat(
                    LLM,
                    chat(&format!("{n} に 1 を足すと？数字だけ答えて。"), 16),
                    |_| {},
                )
                .await
                .unwrap();
            (n, reply.text, start.elapsed())
        }
    };
    let (a, b, c) = tokio::join!(ask(1), ask(2), ask(3));
    for (n, text, at) in [a, b, c] {
        eprintln!("{n}: {text:?} at {:.2} s", at.as_secs_f64());
    }
    assert_eq!(logged(&format!("{LLM} started")).len(), 1);
    drop(invoker);
    wait_for_empty_gpu(&machine).await;
}
