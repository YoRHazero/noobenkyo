//! Helpers for the behaviour tests.
//!
//! The GPU tests read the machines from the JSON file (an `InvokerSettings`)
//! that `NOOBENKYO_INVOKER_TEST_SETTINGS` points at; it stays out of the
//! repository. Its first machine must have the models named below. They
//! share the GPU, so run them one at a time:
//!
//! NOOBENKYO_INVOKER_TEST_SETTINGS=<file> cargo test -p ai-invoker -- --ignored --test-threads=1 --nocapture

use std::process::Stdio;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use ai_invoker::{Invoker, InvokerSettings, Machine};
use serde_json::{Map, Value, json};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub const SETTINGS_VARIABLE: &str = "NOOBENKYO_INVOKER_TEST_SETTINGS";
/// A small GGUF model that loads fast.
pub const LLM: &str = "qwen3.5-4b-q8";
pub const TTS_CLONE: &str = "qwen3-tts-1.7b-base";
pub const TTS_DESIGN: &str = "qwen3-tts-1.7b-voicedesign";

pub fn gpu_settings() -> InvokerSettings {
    let path = std::env::var(SETTINGS_VARIABLE)
        .unwrap_or_else(|_| panic!("{SETTINGS_VARIABLE} is not set"));
    let text = std::fs::read_to_string(&path).unwrap();
    let settings: InvokerSettings = serde_json::from_str(&text).unwrap();
    settings.validate().unwrap();
    settings
}

/// An invoker on the test settings, after `change`, and its first machine.
pub fn gpu_invoker(change: impl FnOnce(&mut InvokerSettings)) -> (Invoker, Machine) {
    init_log();
    forget_log();
    let mut settings = gpu_settings();
    change(&mut settings);
    let machine = settings.machines[0].clone();
    (Invoker::new(settings), machine)
}

/// A chat request as llama-server takes it.
pub fn chat(content: &str, max_tokens: u32) -> Map<String, Value> {
    let body = json!({
        "messages": [{ "role": "user", "content": content }],
        "temperature": 0.7,
        "max_tokens": max_tokens,
    });
    body.as_object().unwrap().clone()
}

/// Prints the invoker's log with the time since the first test began, and
/// keeps the lines for tests that look for one.
pub fn init_log() {
    let logger = LOGGER.get_or_init(|| Printer {
        start: Instant::now(),
        lines: Mutex::new(Vec::new()),
    });
    if log::set_logger(logger).is_ok() {
        log::set_max_level(log::LevelFilter::Info);
    }
}

/// Lines logged since the last `forget_log` that contain `text`, oldest first.
pub fn logged(text: &str) -> Vec<String> {
    let Some(logger) = LOGGER.get() else {
        return vec![];
    };
    let lines = logger.lines.lock().unwrap();
    lines
        .iter()
        .filter(|line| line.contains(text))
        .cloned()
        .collect()
}

/// Makes `logged` look only at what comes next; each test starts with it.
fn forget_log() {
    if let Some(logger) = LOGGER.get() {
        logger.lines.lock().unwrap().clear();
    }
}

static LOGGER: OnceLock<Printer> = OnceLock::new();

struct Printer {
    start: Instant,
    lines: Mutex<Vec<String>>,
}

impl log::Log for Printer {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.target().starts_with("ai_invoker")
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let line = record.args().to_string();
        eprintln!(
            "[{:8.2}] {:5} {line}",
            self.start.elapsed().as_secs_f64(),
            record.level()
        );
        self.lines.lock().unwrap().push(line);
    }

    fn flush(&self) {}
}

/// Runs a bash script on the machine, the way the invoker reaches it.
pub async fn run_on(machine: &Machine, script: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    if let Some(ssh) = &machine.ssh {
        words.extend(["ssh", "-o", "BatchMode=yes", ssh].map(str::to_owned));
    }
    for layer in &machine.command_layers {
        words.extend(layer.args().iter().cloned());
    }
    words.extend(["bash", "-s"].map(str::to_owned));
    let mut child = Command::new(&words[0])
        .args(&words[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(script.as_bytes()).await.unwrap();
    drop(stdin);
    let output = child.wait_with_output().await.unwrap();
    String::from_utf8(output.stdout).unwrap()
}

/// GPU memory in use on the machine, in MiB.
pub async fn gpu_memory(machine: &Machine) -> u32 {
    let output = run_on(
        machine,
        "nvidia-smi --query-gpu=memory.used --format=csv,noheader,nounits\n",
    )
    .await;
    output
        .trim()
        .lines()
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

/// Waits until nothing holds GPU memory and says how long it took.
pub async fn wait_for_empty_gpu(machine: &Machine) -> Duration {
    let start = Instant::now();
    loop {
        let used = gpu_memory(machine).await;
        if used == 0 {
            let waited = start.elapsed();
            eprintln!(
                "GPU memory back to 0 MiB after {:.1} s",
                waited.as_secs_f64()
            );
            return waited;
        }
        assert!(
            start.elapsed() < Duration::from_secs(30),
            "{used} MiB still in use"
        );
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// Channels, sample rate and bits per sample from a WAV file's fmt chunk,
/// and how many seconds its data lasts.
pub fn wav_format(wav: &[u8]) -> (u16, u32, u16, f64) {
    assert_eq!(&wav[0..4], b"RIFF");
    assert_eq!(&wav[8..12], b"WAVE");
    let u16_at = |i: usize| u16::from_le_bytes([wav[i], wav[i + 1]]);
    let u32_at = |i: usize| u32::from_le_bytes([wav[i], wav[i + 1], wav[i + 2], wav[i + 3]]);
    let mut at = 12;
    let mut format = None;
    while at + 8 <= wav.len() {
        let size = u32_at(at + 4) as usize;
        match &wav[at..at + 4] {
            b"fmt " => format = Some((u16_at(at + 10), u32_at(at + 12), u16_at(at + 22))),
            b"data" => {
                let (channels, rate, bits) = format.expect("fmt before data");
                let frame = f64::from(channels) * f64::from(bits / 8);
                return (channels, rate, bits, size as f64 / frame / f64::from(rate));
            }
            _ => {}
        }
        at += 8 + size + size % 2;
    }
    panic!("no data chunk");
}

/// Kills the executor's python on the machine, the way a crash would.
pub async fn kill_executor(machine: &Machine) {
    run_on(machine, "pkill -9 -f noobenkyo-executor/\n").await;
}
