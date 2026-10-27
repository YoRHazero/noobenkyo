use std::process::ExitStatus;

/// How a process ended, for messages the user may read.
pub(crate) fn describe(status: ExitStatus) -> String {
    if let Some(code) = status.code() {
        return format!("退出码 {code}");
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return format!("被信号 {signal} 结束");
        }
    }
    status.to_string()
}
