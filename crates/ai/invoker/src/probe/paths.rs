//! Where things are on a machine by the agreed layout (user's decision):
//! derived from the directories in its ProbeDirs.

use crate::ProbeDirs;

/// llama.cpp's executor and the probe only need the standard library, so
/// this does when nothing else is given.
pub(crate) const SYSTEM_PYTHON: &str = "/usr/bin/python3";

/// qwen-tts's own environment, the only one with torch and qwen_tts.
pub(crate) fn qwen_tts_python(dir: &str) -> String {
    format!("{}/.venv/bin/python", dir.trim_end_matches('/'))
}

pub(crate) fn llama_server(dir: &str) -> String {
    format!("{}/bin/llama-server", dir.trim_end_matches('/'))
}

/// qwen-tts's python when the machine has one, else the one given, else the
/// system's.
pub(crate) fn llama_python(dirs: &ProbeDirs) -> String {
    match (&dirs.qwen_tts, &dirs.llama_python) {
        (Some(dir), _) => qwen_tts_python(dir),
        (None, Some(python)) => python.clone(),
        (None, None) => SYSTEM_PYTHON.to_owned(),
    }
}

/// The pythons the probe tries in turn, each once: qwen-tts's, the one for
/// llama.cpp, the system's.
pub(crate) fn probe_pythons(dirs: &ProbeDirs) -> Vec<String> {
    let mut pythons: Vec<String> = Vec::new();
    let candidates = [
        dirs.qwen_tts.as_deref().map(qwen_tts_python),
        dirs.llama_python.clone(),
        Some(SYSTEM_PYTHON.to_owned()),
    ];
    for python in candidates.into_iter().flatten() {
        if !pythons.contains(&python) {
            pythons.push(python);
        }
    }
    pythons
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dirs(qwen_tts: Option<&str>, llama_python: Option<&str>) -> ProbeDirs {
        ProbeDirs {
            models: "/models".into(),
            qwen_tts: qwen_tts.map(str::to_owned),
            llama_cpp: Some("/opt/llama.cpp/".into()),
            llama_python: llama_python.map(str::to_owned),
        }
    }

    #[test]
    fn follows_the_layout() {
        assert_eq!(
            qwen_tts_python("/opt/qwen-tts/"),
            "/opt/qwen-tts/.venv/bin/python"
        );
        assert_eq!(
            llama_server("/opt/llama.cpp"),
            "/opt/llama.cpp/bin/llama-server"
        );
    }

    #[test]
    fn llama_cpp_runs_on_qwen_tts_python_when_there_is_one() {
        let both = dirs(Some("/opt/qwen-tts"), Some("/usr/local/bin/python3"));
        assert_eq!(llama_python(&both), "/opt/qwen-tts/.venv/bin/python");
        let given = dirs(None, Some("/usr/local/bin/python3"));
        assert_eq!(llama_python(&given), "/usr/local/bin/python3");
        assert_eq!(llama_python(&dirs(None, None)), SYSTEM_PYTHON);
    }

    #[test]
    fn tries_each_python_once() {
        let both = dirs(Some("/opt/qwen-tts"), Some(SYSTEM_PYTHON));
        assert_eq!(
            probe_pythons(&both),
            ["/opt/qwen-tts/.venv/bin/python", SYSTEM_PYTHON]
        );
        assert_eq!(probe_pythons(&ProbeDirs::default()), [SYSTEM_PYTHON]);
    }
}
