// `/job` 参数解析。
// 用户输入统一使用长动作参数；callback payload 仍保持短格式以压缩长度。

/// 任务控制动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum JobAction {
    Pause,
    Resume,
    Stop,
    Status,
}

/// `/job` 参数解析结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct JobArgs {
    pub(super) action: JobAction,
    pub(super) job_id: i64,
}

/// `/job` callback 动作。
///
/// callback 只承载轻量控制，不放链接和长文本，避免 Telegram payload 过长。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum JobCallbackAction {
    Pause,
    Resume,
    StopConfirm,
    Stop,
    Status,
}

/// `/job` callback 解析结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct JobCallbackArgs {
    pub(super) action: JobCallbackAction,
    pub(super) job_id: i64,
}

/// `/job` callback 前缀。
const JOB_CALLBACK_PREFIX: &str = "j:";

/// 解析 `/job <action> <job_id>`。
pub(super) fn parse_job_args(text: &[&str]) -> anyhow::Result<JobArgs> {
    let action = text
        .get(1)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("usage: /job <pause|resume|stop|status> <job_id>"))?;
    let job_id = text
        .get(2)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("usage: /job <pause|resume|stop|status> <job_id>"))?
        .parse::<i64>()?;

    let action = match action {
        "pause" => JobAction::Pause,
        "resume" => JobAction::Resume,
        "stop" | "cancel" => JobAction::Stop,
        "status" => JobAction::Status,
        other => anyhow::bail!("unknown job action: {other}"),
    };

    Ok(JobArgs { action, job_id })
}

/// 判断 callback payload 是否属于 `/job`。
pub(super) fn is_job_callback_data(data: &str) -> bool {
    data.starts_with(JOB_CALLBACK_PREFIX)
}

/// 构造 `/job` callback payload。
///
/// payload 采用 `j:<action>:<job_id>`，短格式便于后续继续加按钮。
pub(super) fn build_job_callback_data(action: JobCallbackAction, job_id: i64) -> String {
    let action = match action {
        JobCallbackAction::Pause => "p",
        JobCallbackAction::Resume => "r",
        JobCallbackAction::StopConfirm => "sc",
        JobCallbackAction::Stop => "s",
        JobCallbackAction::Status => "st",
    };
    format!("{}{}:{}", JOB_CALLBACK_PREFIX, action, job_id)
}

/// 解析 `/job` callback payload。
pub(super) fn parse_job_callback_data(data: &str) -> Option<JobCallbackArgs> {
    let payload = data.strip_prefix(JOB_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    let action = match parts.next()? {
        "p" => JobCallbackAction::Pause,
        "r" => JobCallbackAction::Resume,
        "sc" => JobCallbackAction::StopConfirm,
        "s" => JobCallbackAction::Stop,
        "st" => JobCallbackAction::Status,
        _ => return None,
    };
    let job_id = parts.next()?.parse::<i64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(JobCallbackArgs { action, job_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    // `/job` 只接受长动作参数；短格式只保留在 callback payload 内部。
    #[test]
    fn test_parse_job_args() {
        assert_eq!(
            parse_job_args(&["/job", "pause", "123"]).unwrap(),
            JobArgs {
                action: JobAction::Pause,
                job_id: 123,
            }
        );
        assert_eq!(
            parse_job_args(&["/job", "cancel", "321"]).unwrap(),
            JobArgs {
                action: JobAction::Stop,
                job_id: 321,
            }
        );
        assert_eq!(
            parse_job_args(&["/job", "resume", "654"]).unwrap(),
            JobArgs {
                action: JobAction::Resume,
                job_id: 654,
            }
        );
        assert_eq!(
            parse_job_args(&["/job", "status", "42"]).unwrap(),
            JobArgs {
                action: JobAction::Status,
                job_id: 42,
            }
        );
        assert_eq!(
            parse_job_args(&["/job", "status", "43"]).unwrap(),
            JobArgs {
                action: JobAction::Status,
                job_id: 43,
            }
        );
        assert!(parse_job_args(&["/job", "bad", "1"]).is_err());
        assert!(parse_job_args(&["/job", "pause"]).is_err());
        assert!(parse_job_args(&["/job", "pause", "abc"]).is_err());
    }

    // callback payload 使用短格式，避免 Telegram callback data 过长。
    #[test]
    fn test_job_callback_data_roundtrip() {
        let data = build_job_callback_data(JobCallbackAction::Status, 42);
        assert_eq!(data, "j:st:42");
        assert!(is_job_callback_data(&data));
        assert_eq!(
            parse_job_callback_data(&data),
            Some(JobCallbackArgs {
                action: JobCallbackAction::Status,
                job_id: 42,
            })
        );

        let confirm_stop = build_job_callback_data(JobCallbackAction::StopConfirm, 42);
        assert_eq!(confirm_stop, "j:sc:42");
        assert_eq!(
            parse_job_callback_data(&confirm_stop),
            Some(JobCallbackArgs {
                action: JobCallbackAction::StopConfirm,
                job_id: 42,
            })
        );

        // 历史消息上的旧停止按钮仍然能解析成真正停止，避免旧 callback 失效。
        assert_eq!(
            parse_job_callback_data("j:s:42"),
            Some(JobCallbackArgs {
                action: JobCallbackAction::Stop,
                job_id: 42,
            })
        );

        assert_eq!(parse_job_callback_data("d:r:run:8:1"), None);
        assert_eq!(parse_job_callback_data("j:x:42"), None);
        assert_eq!(parse_job_callback_data("j:st:not-int"), None);
    }
}
