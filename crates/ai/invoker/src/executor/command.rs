use std::process::Stdio;

use tokio::process::{Child, Command};

use crate::Machine;

/// A command to run for a machine: over ssh when it has one, inside its
/// command layers. Only paths go on the command line; everything else goes
/// over stdin, untouched by the quoting of the remote cmd or shell.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CommandLine {
    pub program: String,
    pub args: Vec<String>,
}

impl CommandLine {
    /// `python -`, which runs the script sent on stdin.
    pub(crate) fn script(machine: &Machine, python: &str, connect_timeout_secs: u64) -> Self {
        Self::on(machine, &[python, "-"], connect_timeout_secs)
    }

    /// `python <dir>`, which runs the executor's `__main__.py`.
    pub(crate) fn executor(
        machine: &Machine,
        python: &str,
        dir: &str,
        connect_timeout_secs: u64,
    ) -> Self {
        Self::on(machine, &[python, dir], connect_timeout_secs)
    }

    fn on(machine: &Machine, inner: &[&str], connect_timeout_secs: u64) -> Self {
        let mut words: Vec<String> = Vec::new();
        if let Some(target) = &machine.ssh {
            words.extend(
                [
                    "ssh",
                    "-o",
                    "BatchMode=yes",
                    "-o",
                    &format!("ConnectTimeout={connect_timeout_secs}"),
                    target,
                ]
                .map(str::to_owned),
            );
        }
        for layer in &machine.command_layers {
            words.extend(layer.args().iter().cloned());
        }
        words.extend(inner.iter().map(|word| (*word).to_owned()));
        let program = words.remove(0);
        Self {
            program,
            args: words,
        }
    }

    /// Starts it with all three pipes. Killed if dropped, so an abandoned
    /// start closes the SSH connection and the remote side sees EOF.
    pub(crate) fn spawn(&self) -> Result<Child, String> {
        Command::new(&self.program)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|error| format!("启动 {} 失败：{error}", self.program))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CommandLayer;

    fn machine(ssh: Option<&str>, layers: &[&str]) -> Machine {
        Machine {
            name: "m".into(),
            ssh: ssh.map(str::to_owned),
            command_layers: layers.iter().map(|l| CommandLayer::parse(l)).collect(),
            runtimes: vec![],
            probe: None,
        }
    }

    #[test]
    fn goes_over_ssh_and_through_the_layers() {
        let yorha = machine(Some("yorha@192.0.2.10"), &["wsl -e"]);
        let command = CommandLine::executor(&yorha, "/venv/bin/python", "/home/zero/.cache/x", 5);
        assert_eq!(command.program, "ssh");
        assert_eq!(
            command.args,
            [
                "-o",
                "BatchMode=yes",
                "-o",
                "ConnectTimeout=5",
                "yorha@192.0.2.10",
                "wsl",
                "-e",
                "/venv/bin/python",
                "/home/zero/.cache/x"
            ]
        );
        let script = CommandLine::script(&yorha, "/usr/bin/python3", 9);
        assert_eq!(script.args[3], "ConnectTimeout=9");
        assert_eq!(&script.args[7..], ["/usr/bin/python3", "-"]);
    }

    #[test]
    fn runs_here_without_ssh() {
        let local = machine(None, &[]);
        let command = CommandLine::script(&local, "/usr/bin/python3", 5);
        assert_eq!(
            command,
            CommandLine {
                program: "/usr/bin/python3".into(),
                args: vec!["-".into()]
            }
        );
        let wrapped = CommandLine::script(&machine(None, &["docker exec -i gpu"]), "/py", 5);
        assert_eq!(wrapped.program, "docker");
        assert_eq!(wrapped.args, ["exec", "-i", "gpu", "/py", "-"]);
    }
}
