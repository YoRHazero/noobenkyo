use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InvokeError {
    /// No machine could take the call: none has the model for this call, or
    /// every one that has it failed to start an executor.
    #[error("{}", no_machine(.model, .tried))]
    NoMachine {
        model: String,
        /// Each machine tried and why it failed.
        tried: Vec<(String, String)>,
    },
    /// The executor took the call and failed it, or died on the way.
    #[error("{machine} 上的执行端出错：{detail}")]
    ExecutorFailed { machine: String, detail: String },
}

fn no_machine(model: &str, tried: &[(String, String)]) -> String {
    if tried.is_empty() {
        return format!("没有能运行 {model} 的机器");
    }
    let reasons: Vec<String> = tried
        .iter()
        .map(|(machine, reason)| format!("{machine}：{reason}"))
        .collect();
    format!("没有能运行 {model} 的机器（{}）", reasons.join("；"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_each_machine_tried() {
        let none = InvokeError::NoMachine {
            model: "m".into(),
            tried: vec![],
        };
        assert_eq!(none.to_string(), "没有能运行 m 的机器");
        let two = InvokeError::NoMachine {
            model: "m".into(),
            tried: vec![
                ("a".into(), "连不上".into()),
                ("b".into(), "显存不够".into()),
            ],
        };
        assert_eq!(
            two.to_string(),
            "没有能运行 m 的机器（a：连不上；b：显存不够）"
        );
    }
}
