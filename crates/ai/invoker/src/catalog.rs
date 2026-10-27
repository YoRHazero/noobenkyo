//! Which machines can take a call, read from the settings alone.

use crate::protocol::StartRuntime;
use crate::{InvokeError, InvokerSettings, Runtime, TtsOp};

/// The calls a model can take; named as in the protocol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Operation {
    LlmChat,
    TtsClone,
    TtsDesign,
}

impl Operation {
    fn name(self) -> &'static str {
        match self {
            Operation::LlmChat => "llm_chat",
            Operation::TtsClone => "tts_clone",
            Operation::TtsDesign => "tts_design",
        }
    }

    fn of(op: TtsOp) -> Self {
        match op {
            TtsOp::TtsClone => Operation::TtsClone,
            TtsOp::TtsDesign => Operation::TtsDesign,
        }
    }
}

/// A machine with the model for the call, and how to start it there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Candidate {
    /// Index in the settings, which is the order of preference.
    pub machine: usize,
    pub python: String,
    pub start: StartRuntime,
}

/// Every machine that has `model` and can do `operation` with it. None at
/// all is `NoMachine`, listing the machines whose model is for another call.
pub(crate) fn candidates(
    settings: &InvokerSettings,
    model: &str,
    operation: Operation,
) -> Result<Vec<Candidate>, InvokeError> {
    let mut found = Vec::new();
    let mut wrong_call = Vec::new();
    for (index, machine) in settings.machines.iter().enumerate() {
        let Some((can, python, start)) = machine.runtimes.iter().find_map(|r| find(r, model))
        else {
            continue;
        };
        if can == operation {
            found.push(Candidate {
                machine: index,
                python,
                start,
            });
        } else {
            let reason = format!("{model} 只能做 {}，不能做 {}", can.name(), operation.name());
            wrong_call.push((machine.name.clone(), reason));
        }
    }
    if found.is_empty() {
        return Err(InvokeError::NoMachine {
            model: model.to_owned(),
            tried: wrong_call,
        });
    }
    Ok(found)
}

/// The call `model` takes in this runtime, and the python and `start` that
/// run it, if it is there.
fn find(runtime: &Runtime, model: &str) -> Option<(Operation, String, StartRuntime)> {
    let python = runtime.python().to_owned();
    match runtime {
        Runtime::LlamaCpp { server, models, .. } => {
            let found = models.iter().find(|m| m.name == model)?;
            let start = StartRuntime::LlamaCpp {
                server: server.clone(),
                path: found.path.clone(),
                args: found.args.clone(),
            };
            Some((Operation::LlmChat, python, start))
        }
        Runtime::QwenTts { models, .. } => {
            let found = models.iter().find(|m| m.name == model)?;
            let start = StartRuntime::QwenTts {
                path: found.path.clone(),
            };
            Some((Operation::of(found.op), python, start))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LlamaCppModel, Machine, QwenTtsModel};

    fn machine(name: &str, llm: &[&str], tts: &[(&str, TtsOp)]) -> Machine {
        Machine {
            name: name.into(),
            ssh: Some(name.into()),
            command_layers: vec![],
            runtimes: vec![
                Runtime::LlamaCpp {
                    python: "/usr/bin/python3".into(),
                    server: format!("/{name}/llama-server"),
                    models: llm
                        .iter()
                        .map(|m| LlamaCppModel {
                            name: (*m).into(),
                            path: format!("/{name}/{m}.gguf"),
                            args: vec!["-c".into(), "4096".into()],
                        })
                        .collect(),
                },
                Runtime::QwenTts {
                    python: "/venv/bin/python".into(),
                    models: tts
                        .iter()
                        .map(|(m, op)| QwenTtsModel {
                            name: (*m).into(),
                            op: *op,
                            path: format!("/{name}/{m}"),
                        })
                        .collect(),
                },
            ],
        }
    }

    fn settings() -> InvokerSettings {
        InvokerSettings {
            machines: vec![
                machine("a", &["llm"], &[("base", TtsOp::TtsClone)]),
                machine("b", &[], &[("design", TtsOp::TtsDesign)]),
                machine("c", &["llm"], &[]),
            ],
            ..InvokerSettings::default()
        }
    }

    #[test]
    fn lists_the_machines_in_order_with_their_own_paths() {
        let found = candidates(&settings(), "llm", Operation::LlmChat).unwrap();
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].machine, 0);
        assert_eq!(found[1].machine, 2);
        assert_eq!(found[1].python, "/usr/bin/python3");
        assert_eq!(
            found[1].start,
            StartRuntime::LlamaCpp {
                server: "/c/llama-server".into(),
                path: "/c/llm.gguf".into(),
                args: vec!["-c".into(), "4096".into()]
            }
        );
        let design = candidates(&settings(), "design", Operation::TtsDesign).unwrap();
        assert_eq!(design[0].machine, 1);
        assert_eq!(design[0].python, "/venv/bin/python");
        assert_eq!(
            design[0].start,
            StartRuntime::QwenTts {
                path: "/b/design".into()
            }
        );
    }

    #[test]
    fn an_unknown_model_has_no_machine() {
        assert_eq!(
            candidates(&settings(), "typo", Operation::LlmChat),
            Err(InvokeError::NoMachine {
                model: "typo".into(),
                tried: vec![]
            })
        );
    }

    #[test]
    fn a_model_for_another_call_is_refused_without_starting_anything() {
        assert_eq!(
            candidates(&settings(), "base", Operation::LlmChat),
            Err(InvokeError::NoMachine {
                model: "base".into(),
                tried: vec![("a".into(), "base 只能做 tts_clone，不能做 llm_chat".into())]
            })
        );
        assert!(candidates(&settings(), "llm", Operation::TtsClone).is_err());
        assert!(candidates(&settings(), "base", Operation::TtsDesign).is_err());
    }
}
