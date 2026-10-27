use std::time::Duration;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::catalog::{self, Candidate, Operation};
use crate::executor::{Bundle, Executor, Launch, Reply, launch};
use crate::protocol::{LlmDone, Request, TtsDone};
use crate::schedule::{Lease, State};
use crate::{
    InvokeError, InvokerSettings, LlmProgress, LlmReply, TtsCloneRequest, TtsDesignRequest,
    TtsReply,
};

/// Runs model calls on the machines in its settings. Core makes one at
/// startup and the whole app shares it. Its settings never change: new
/// settings mean a new Invoker, and dropping this one closes its executors.
pub struct Invoker {
    settings: InvokerSettings,
    state: State,
    bundle: Bundle,
}

impl Invoker {
    /// Takes settings that have passed `validate`; reads no file itself.
    pub fn new(settings: InvokerSettings) -> Invoker {
        Invoker {
            state: State::new(settings.machines.len()),
            settings,
            bundle: Bundle::new(),
        }
    }

    /// Sends `body`, a llama-server chat request as the caller wrote it, to
    /// `model`. Dropping the future cancels the call.
    pub async fn llm_chat(
        &self,
        model: &str,
        body: Map<String, Value>,
        progress: impl Fn(LlmProgress),
    ) -> Result<LlmReply, InvokeError> {
        let request = Request::LlmChat { body };
        let (fields, machine) = self
            .invoke(model, Operation::LlmChat, &request, |tokens| {
                progress(LlmProgress { tokens })
            })
            .await?;
        let done: LlmDone = parse_done(fields, &machine)?;
        Ok(LlmReply {
            text: done.text,
            tokens: done.tokens,
            finish: done.finish,
            machine,
        })
    }

    /// Says `text` in the voice of the reference recording. Dropping the
    /// future cancels the call, though the sentence under way is finished.
    pub async fn tts_clone(
        &self,
        model: &str,
        request: TtsCloneRequest,
    ) -> Result<TtsReply, InvokeError> {
        let request = Request::TtsClone {
            text: request.text,
            language: request.language,
            ref_audio: BASE64.encode(&request.ref_audio),
            ref_text: request.ref_text,
        };
        self.tts(model, Operation::TtsClone, &request).await
    }

    /// Says `text` in a voice made up from `instruct`.
    pub async fn tts_design(
        &self,
        model: &str,
        request: TtsDesignRequest,
    ) -> Result<TtsReply, InvokeError> {
        let request = Request::TtsDesign {
            text: request.text,
            language: request.language,
            instruct: request.instruct,
            seed: request.seed,
        };
        self.tts(model, Operation::TtsDesign, &request).await
    }

    async fn tts(
        &self,
        model: &str,
        operation: Operation,
        request: &Request,
    ) -> Result<TtsReply, InvokeError> {
        let (fields, machine) = self.invoke(model, operation, request, |_| {}).await?;
        let done: TtsDone = parse_done(fields, &machine)?;
        let wav = BASE64
            .decode(done.wav)
            .map_err(|error| failed(&machine, format!("wav 不是 base64：{error}")))?;
        Ok(TtsReply { wav, machine })
    }

    /// Runs one call on a machine that has the model, trying the next one
    /// when an executor fails to start. Returns the `done` fields and where
    /// it ran.
    async fn invoke(
        &self,
        model: &str,
        operation: Operation,
        request: &Request,
        progress: impl Fn(u32),
    ) -> Result<(Map<String, Value>, String), InvokeError> {
        let candidates = catalog::candidates(&self.settings, model, operation)?;
        let mut tried: Vec<(usize, String)> = Vec::new();
        loop {
            let untried: Vec<usize> = candidates
                .iter()
                .map(|candidate| candidate.machine)
                .filter(|machine| tried.iter().all(|(failed, _)| failed != machine))
                .collect();
            if untried.is_empty() {
                let tried = tried
                    .into_iter()
                    .map(|(machine, reason)| (self.settings.machines[machine].name.clone(), reason))
                    .collect();
                return Err(InvokeError::NoMachine {
                    model: model.to_owned(),
                    tried,
                });
            }
            let lease = self.state.lease(&untried, model).await;
            let candidate = candidates
                .iter()
                .find(|candidate| candidate.machine == lease.machine())
                .expect("leased among the candidates");
            let name = &self.settings.machines[candidate.machine].name;
            let executor = match self.executor(&lease, candidate, model).await {
                Ok(executor) => executor,
                Err(reason) => {
                    tried.push((candidate.machine, reason));
                    continue;
                }
            };
            let mut call = executor.call(request);
            loop {
                match call.next().await {
                    Reply::Progress(tokens) => progress(tokens),
                    Reply::Done(fields) => return Ok((fields, name.clone())),
                    Reply::Error(detail) | Reply::Failed(detail) => {
                        return Err(failed(name, detail));
                    }
                    // It closed just before the request got there: start it again.
                    Reply::Gone => break,
                }
            }
        }
    }

    /// The machine's executor for `model`, started if need be. An executor
    /// with another model is closed first, so its GPU memory is free.
    async fn executor(
        &self,
        lease: &Lease<'_>,
        candidate: &Candidate,
        model: &str,
    ) -> Result<Executor, String> {
        if let Some(executor) = lease.executor_for(model) {
            return Ok(executor);
        }
        let machine = &self.settings.machines[candidate.machine];
        if let Some(old) = lease.take_executor() {
            if old.is_alive() {
                log::info!("{}: closing {} to start {model}", machine.name, old.model());
            }
            old.shutdown().await;
        }
        let executor = launch(
            Launch {
                machine,
                python: &candidate.python,
                model,
                start: &candidate.start,
                connect_timeout_secs: self.settings.connect_timeout_secs,
                idle_timeout: Duration::from_secs(self.settings.idle_timeout_secs),
            },
            &self.bundle,
        )
        .await?;
        lease.set_executor(executor.clone());
        Ok(executor)
    }
}

fn parse_done<T: DeserializeOwned>(
    fields: Map<String, Value>,
    machine: &str,
) -> Result<T, InvokeError> {
    serde_json::from_value(Value::Object(fields))
        .map_err(|error| failed(machine, format!("执行端的回复不对：{error}")))
}

fn failed(machine: &str, detail: String) -> InvokeError {
    InvokeError::ExecutorFailed {
        machine: machine.to_owned(),
        detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TtsLanguage;

    fn assert_send<T: Send>(_: &T) {}

    #[test]
    fn calls_can_be_awaited_on_any_thread() {
        let invoker = Invoker::new(InvokerSettings::default());
        assert_send(&invoker.llm_chat("m", Map::new(), |_| {}));
        let clone = TtsCloneRequest {
            text: String::new(),
            language: TtsLanguage::Japanese,
            ref_audio: vec![],
            ref_text: String::new(),
        };
        assert_send(&invoker.tts_clone("m", clone));
    }
}
