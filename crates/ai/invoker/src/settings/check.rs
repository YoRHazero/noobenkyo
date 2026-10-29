//! What `update_settings` checks before taking new invoker settings. Whether
//! the files exist is only found out when an executor starts.

use thiserror::Error;

use super::{InvokerSettings, Machine, Runtime};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InvokerSettingsError {
    #[error("闲置超时至少 1 秒")]
    IdleTimeout,
    #[error("连接超时至少 1 秒")]
    ConnectTimeout,
    #[error("机器名不能为空")]
    EmptyMachineName,
    #[error("机器名重复：{0}")]
    DuplicateMachine(String),
    #[error("{machine} 的 ssh 只能含字母、数字和 ._-/@:=+，不能以 - 开头：{ssh}")]
    BadSsh { machine: String, ssh: String },
    #[error("{machine} 有一层命令是空的")]
    EmptyCommandLayer { machine: String },
    #[error("{machine} 上有模型名是空的")]
    EmptyModelName { machine: String },
    #[error("{machine} 上的模型名重复：{model}")]
    DuplicateModel { machine: String, model: String },
    #[error("{machine} 的 {what} 要写绝对路径（以 / 开头）：{path}")]
    NotAbsolute {
        machine: String,
        what: String,
        path: String,
    },
    #[error("{machine} 的命令行上只能出现字母、数字和 ._-/@:=+：{value}")]
    UnsafeOnCommandLine { machine: String, value: String },
}

impl InvokerSettings {
    pub fn validate(&self) -> Result<(), InvokerSettingsError> {
        if self.idle_timeout_secs == 0 {
            return Err(InvokerSettingsError::IdleTimeout);
        }
        if self.connect_timeout_secs == 0 {
            return Err(InvokerSettingsError::ConnectTimeout);
        }
        for (index, machine) in self.machines.iter().enumerate() {
            if machine.name.is_empty() {
                return Err(InvokerSettingsError::EmptyMachineName);
            }
            if self.machines[..index]
                .iter()
                .any(|other| other.name == machine.name)
            {
                return Err(InvokerSettingsError::DuplicateMachine(machine.name.clone()));
            }
            check_machine(machine)?;
        }
        Ok(())
    }
}

/// One machine's checks, also run before probing one not saved yet.
pub(crate) fn check_machine(machine: &Machine) -> Result<(), InvokerSettingsError> {
    let name = || machine.name.clone();
    if let Some(ssh) = &machine.ssh
        && (!is_safe_on_command_line(ssh) || ssh.starts_with('-'))
    {
        return Err(InvokerSettingsError::BadSsh {
            machine: name(),
            ssh: ssh.clone(),
        });
    }
    for layer in &machine.command_layers {
        if layer.args().is_empty() {
            return Err(InvokerSettingsError::EmptyCommandLayer { machine: name() });
        }
        for arg in layer.args() {
            check_command_line(machine, arg)?;
        }
    }
    check_probe(machine)?;
    let mut models: Vec<&str> = Vec::new();
    for runtime in &machine.runtimes {
        check_absolute(machine, "python", runtime.python())?;
        check_command_line(machine, runtime.python())?;
        let paths: Vec<(&str, &str)> = match runtime {
            Runtime::LlamaCpp { server, models, .. } => {
                check_absolute(machine, "server", server)?;
                models
                    .iter()
                    .map(|m| (m.name.as_str(), m.path.as_str()))
                    .collect()
            }
            Runtime::QwenTts { models, .. } => models
                .iter()
                .map(|m| (m.name.as_str(), m.path.as_str()))
                .collect(),
        };
        for (model, path) in paths {
            if model.is_empty() {
                return Err(InvokerSettingsError::EmptyModelName { machine: name() });
            }
            if models.contains(&model) {
                return Err(InvokerSettingsError::DuplicateModel {
                    machine: name(),
                    model: model.to_owned(),
                });
            }
            models.push(model);
            check_absolute(machine, &format!("模型 {model} 的 path"), path)?;
        }
    }
    Ok(())
}

/// The probe's directories. qwen-tts's python and the one for llama.cpp go
/// on the command line; the rest only goes over stdin.
fn check_probe(machine: &Machine) -> Result<(), InvokerSettingsError> {
    let Some(dirs) = &machine.probe else {
        return Ok(());
    };
    check_absolute(machine, "模型文件夹", &dirs.models)?;
    if let Some(dir) = &dirs.qwen_tts {
        check_absolute(machine, "qwen-tts 目录", dir)?;
        check_command_line(machine, dir)?;
    }
    if let Some(dir) = &dirs.llama_cpp {
        check_absolute(machine, "llama.cpp 目录", dir)?;
    }
    if let Some(python) = &dirs.llama_python {
        check_absolute(machine, "llama.cpp 的 python", python)?;
        check_command_line(machine, python)?;
    }
    Ok(())
}

/// Executors only run on Linux, and nothing on the way expands `~`.
fn check_absolute(machine: &Machine, what: &str, path: &str) -> Result<(), InvokerSettingsError> {
    if path.starts_with('/') {
        return Ok(());
    }
    Err(InvokerSettingsError::NotAbsolute {
        machine: machine.name.clone(),
        what: what.to_owned(),
        path: path.to_owned(),
    })
}

/// The command line passes through the remote cmd or shell, whose quoting
/// rules differ, so it only carries characters none of them treat specially.
fn check_command_line(machine: &Machine, value: &str) -> Result<(), InvokerSettingsError> {
    if is_safe_on_command_line(value) {
        return Ok(());
    }
    Err(InvokerSettingsError::UnsafeOnCommandLine {
        machine: machine.name.clone(),
        value: value.to_owned(),
    })
}

pub(crate) fn is_safe_on_command_line(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._-/@:=+".contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{CommandLayer, LlamaCppModel, ProbeDirs, QwenTtsModel, TtsOp};

    fn yorha() -> Machine {
        Machine {
            name: "yorha".into(),
            ssh: Some("yorha@192.0.2.10".into()),
            command_layers: vec![CommandLayer::parse("wsl -e")],
            runtimes: vec![
                Runtime::LlamaCpp {
                    python: "/usr/bin/python3".into(),
                    server: "/home/zero/opt/llama.cpp/bin/llama-server".into(),
                    models: vec![LlamaCppModel {
                        name: "qwen3.5-9b-q8".into(),
                        path: "/home/zero/models/My Models/Qwen3.5-9B-Q8_0.gguf".into(),
                        args: vec!["-c".into(), "16384".into()],
                    }],
                },
                Runtime::QwenTts {
                    python: "/home/zero/opt/qwen-tts/.venv/bin/python".into(),
                    models: vec![QwenTtsModel {
                        name: "qwen3-tts-1.7b-base".into(),
                        op: TtsOp::TtsClone,
                        path: "/home/zero/models/Qwen3-TTS-12Hz-1.7B-Base".into(),
                    }],
                },
            ],
            probe: Some(ProbeDirs {
                models: "/home/zero/models".into(),
                qwen_tts: Some("/home/zero/opt/qwen-tts".into()),
                llama_cpp: Some("/home/zero/opt/llama.cpp".into()),
                llama_python: None,
            }),
        }
    }

    fn with(machines: Vec<Machine>) -> InvokerSettings {
        InvokerSettings {
            machines,
            ..InvokerSettings::default()
        }
    }

    fn error_of(change: impl FnOnce(&mut Machine)) -> InvokerSettingsError {
        let mut machine = yorha();
        change(&mut machine);
        with(vec![machine]).validate().unwrap_err()
    }

    #[test]
    fn checks_the_probe_directories() {
        let error = error_of(|m| m.probe.as_mut().unwrap().models = "models".into());
        assert_eq!(
            error,
            InvokerSettingsError::NotAbsolute {
                machine: "yorha".into(),
                what: "模型文件夹".into(),
                path: "models".into(),
            }
        );
        let error = error_of(|m| m.probe.as_mut().unwrap().qwen_tts = Some("/opt/qwen tts".into()));
        assert_eq!(
            error,
            InvokerSettingsError::UnsafeOnCommandLine {
                machine: "yorha".into(),
                value: "/opt/qwen tts".into(),
            }
        );
        // The models only go over stdin, so they may have spaces.
        let spaced = |m: &mut Machine| m.probe.as_mut().unwrap().models = "/My Models".into();
        let mut machine = yorha();
        spaced(&mut machine);
        assert_eq!(with(vec![machine]).validate(), Ok(()));
    }

    #[test]
    fn accepts_the_default_and_a_full_machine() {
        assert_eq!(InvokerSettings::default().validate(), Ok(()));
        assert_eq!(with(vec![yorha()]).validate(), Ok(()));
        let local = Machine {
            ssh: None,
            command_layers: vec![],
            ..yorha()
        };
        assert_eq!(with(vec![local]).validate(), Ok(()));
    }

    #[test]
    fn reads_the_documented_json() {
        let json = r#"{
            "machines": [{ "name": "yorha", "ssh": "yorha@192.0.2.10", "command_layers": ["wsl -e"],
              "runtimes": [
                { "kind": "LlamaCpp", "python": "/usr/bin/python3", "server": "/opt/llama-server",
                  "models": [{ "name": "m", "path": "/m.gguf", "args": ["-c", "16384"] }] },
                { "kind": "QwenTts", "python": "/venv/bin/python",
                  "models": [{ "name": "t", "op": "TtsDesign", "path": "/t" }] }
              ] }],
            "idle_timeout_secs": 600,
            "connect_timeout_secs": 5
        }"#;
        let settings: InvokerSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.machines[0].command_layers[0].args(), ["wsl", "-e"]);
        assert!(matches!(
            &settings.machines[0].runtimes[1],
            Runtime::QwenTts { models, .. } if models[0].op == TtsOp::TtsDesign
        ));
        let back = serde_json::to_value(&settings).unwrap();
        let again: InvokerSettings = serde_json::from_value(back).unwrap();
        assert_eq!(again, settings);
    }

    #[test]
    fn rejects_zero_timeouts() {
        let idle = InvokerSettings {
            idle_timeout_secs: 0,
            ..InvokerSettings::default()
        };
        assert_eq!(idle.validate(), Err(InvokerSettingsError::IdleTimeout));
        let connect = InvokerSettings {
            connect_timeout_secs: 0,
            ..InvokerSettings::default()
        };
        assert_eq!(
            connect.validate(),
            Err(InvokerSettingsError::ConnectTimeout)
        );
    }

    #[test]
    fn machine_names_are_present_and_unique() {
        assert_eq!(
            error_of(|m| m.name.clear()),
            InvokerSettingsError::EmptyMachineName
        );
        assert_eq!(
            with(vec![yorha(), yorha()]).validate(),
            Err(InvokerSettingsError::DuplicateMachine("yorha".into()))
        );
    }

    #[test]
    fn model_names_are_unique_within_a_machine_across_runtimes() {
        let error = error_of(|m| {
            if let Runtime::QwenTts { models, .. } = &mut m.runtimes[1] {
                models[0].name = "qwen3.5-9b-q8".into();
            }
        });
        assert_eq!(
            error,
            InvokerSettingsError::DuplicateModel {
                machine: "yorha".into(),
                model: "qwen3.5-9b-q8".into()
            }
        );
        let mut other = yorha();
        other.name = "zero".into();
        assert_eq!(with(vec![yorha(), other]).validate(), Ok(()));
        let error = error_of(|m| {
            if let Runtime::QwenTts { models, .. } = &mut m.runtimes[1] {
                models[0].name.clear();
            }
        });
        assert_eq!(
            error,
            InvokerSettingsError::EmptyModelName {
                machine: "yorha".into()
            }
        );
    }

    #[test]
    fn paths_are_absolute() {
        let error = error_of(|m| {
            if let Runtime::LlamaCpp { server, .. } = &mut m.runtimes[0] {
                *server = "~/opt/llama.cpp/bin/llama-server".into();
            }
        });
        assert!(
            matches!(error, InvokerSettingsError::NotAbsolute { what, .. } if what == "server")
        );
        let error = error_of(|m| {
            if let Runtime::QwenTts { models, .. } = &mut m.runtimes[1] {
                models[0].path = "models/base".into();
            }
        });
        assert!(matches!(
            error,
            InvokerSettingsError::NotAbsolute { what, .. } if what == "模型 qwen3-tts-1.7b-base 的 path"
        ));
        let error = error_of(|m| {
            if let Runtime::QwenTts { python, .. } = &mut m.runtimes[1] {
                *python = "python3".into();
            }
        });
        assert!(
            matches!(error, InvokerSettingsError::NotAbsolute { what, .. } if what == "python")
        );
    }

    #[test]
    fn only_plain_characters_go_on_the_command_line() {
        let error = error_of(|m| {
            if let Runtime::LlamaCpp { python, .. } = &mut m.runtimes[0] {
                *python = "/opt/my python/bin/python3".into();
            }
        });
        assert!(matches!(
            error,
            InvokerSettingsError::UnsafeOnCommandLine { .. }
        ));
        let error = error_of(|m| m.command_layers = vec![CommandLayer::parse("wsl -e \"x\"")]);
        assert_eq!(
            error,
            InvokerSettingsError::UnsafeOnCommandLine {
                machine: "yorha".into(),
                value: "\"x\"".into()
            }
        );
        let error = error_of(|m| m.command_layers = vec![CommandLayer::parse("  ")]);
        assert_eq!(
            error,
            InvokerSettingsError::EmptyCommandLayer {
                machine: "yorha".into()
            }
        );
        // The server and model paths go over stdin, so spaces are fine there.
        assert_eq!(with(vec![yorha()]).validate(), Ok(()));
    }

    #[test]
    fn ssh_is_one_plain_word_that_is_not_an_option() {
        for ssh in ["-oProxyCommand=x", "yorha host", ""] {
            let error = error_of(|m| m.ssh = Some(ssh.into()));
            assert!(
                matches!(error, InvokerSettingsError::BadSsh { .. }),
                "{ssh}"
            );
        }
        assert_eq!(
            with(vec![Machine {
                ssh: Some("zero".into()),
                ..yorha()
            }])
            .validate(),
            Ok(())
        );
    }
}
