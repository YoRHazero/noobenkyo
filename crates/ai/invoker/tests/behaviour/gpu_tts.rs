//! tts_design and tts_clone on the GPU machine, and switching between models.

use std::time::{Duration, Instant};

use ai_invoker::{LlmFinish, TtsCloneRequest, TtsDesignRequest, TtsLanguage};

use crate::support::{
    LLM, TTS_CLONE, TTS_DESIGN, chat, gpu_invoker, gpu_memory, logged, wait_for_empty_gpu,
    wav_format,
};

const REFERENCE: &str = "はじめまして。今日はいい天気ですね。散歩に行きませんか。";

fn design() -> TtsDesignRequest {
    TtsDesignRequest {
        text: REFERENCE.into(),
        language: TtsLanguage::Japanese,
        instruct: "落ち着いた二十代の女性の声。ゆっくり、はっきり話す。".into(),
        seed: Some(7),
    }
}

fn clone(text: &str, ref_audio: Vec<u8>) -> TtsCloneRequest {
    TtsCloneRequest {
        text: text.into(),
        language: TtsLanguage::Japanese,
        ref_audio,
        ref_text: REFERENCE.into(),
    }
}

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn design_a_voice_clone_it_and_switch_models_on_one_machine() {
    let (invoker, machine) = gpu_invoker(|_| {});
    let start = Instant::now();
    let designed = invoker.tts_design(TTS_DESIGN, design()).await.unwrap();
    let (channels, rate, bits, seconds) = wav_format(&designed.wav);
    eprintln!(
        "tts_design (start included) {:.1} s: {channels} channel, {rate} Hz, {bits} bit, {seconds:.1} s of audio",
        start.elapsed().as_secs_f64()
    );
    assert_eq!((channels, rate, bits), (1, 24000, 16));
    assert!(seconds > 1.0);
    assert_eq!(designed.machine, machine.name);
    eprintln!(
        "GPU memory with {TTS_DESIGN}: {} MiB",
        gpu_memory(&machine).await
    );

    let start = Instant::now();
    let cloned = invoker
        .tts_clone(TTS_CLONE, clone("おはようございます。", designed.wav))
        .await
        .unwrap();
    let (channels, rate, bits, seconds) = wav_format(&cloned.wav);
    eprintln!(
        "tts_clone after switching (start included) {:.1} s: {seconds:.1} s of audio",
        start.elapsed().as_secs_f64()
    );
    assert_eq!((channels, rate, bits), (1, 24000, 16));
    eprintln!(
        "GPU memory with {TTS_CLONE}: {} MiB",
        gpu_memory(&machine).await
    );

    let start = Instant::now();
    let reply = invoker
        .llm_chat(LLM, chat("「はい」とだけ答えてください。", 16), |_| {})
        .await
        .unwrap();
    eprintln!(
        "llm_chat after switching runtimes {:.1} s: {:?}",
        start.elapsed().as_secs_f64(),
        reply.text
    );
    assert_eq!(reply.finish, LlmFinish::Stop);
    assert_eq!(logged("closing").len(), 2);
    drop(invoker);
    wait_for_empty_gpu(&machine).await;
}

#[tokio::test]
#[ignore = "needs the GPU machine; see support.rs"]
async fn a_dropped_sentence_is_finished_before_the_next_one() {
    let (invoker, machine) = gpu_invoker(|_| {});
    let reference = invoker.tts_design(TTS_DESIGN, design()).await.unwrap().wav;
    let warm = Instant::now();
    invoker
        .tts_clone(TTS_CLONE, clone("おはようございます。", reference.clone()))
        .await
        .unwrap();
    eprintln!("clone with start {:.1} s", warm.elapsed().as_secs_f64());

    let long =
        "昔々、ある山の奥に小さな村がありました。村の人たちは毎朝早く起きて、畑で働きました。";
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        invoker.tts_clone(TTS_CLONE, clone(long, reference.clone())),
    )
    .await;
    assert!(result.is_err(), "finished within a second");
    let cancelled = Instant::now();
    invoker
        .tts_clone(TTS_CLONE, clone("はい。", reference))
        .await
        .unwrap();
    eprintln!(
        "next sentence done {:.1} s after the cancel",
        cancelled.elapsed().as_secs_f64()
    );
    assert_eq!(logged("after its cancel").len(), 1);
    drop(invoker);
    wait_for_empty_gpu(&machine).await;
}
