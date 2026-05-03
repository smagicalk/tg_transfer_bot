// `/job` 参数解析。
// 短命令和长命令共用英文参数，避免 Telegram 里输入过长。

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
        "pause" | "p" => JobAction::Pause,
        "resume" | "r" => JobAction::Resume,
        "stop" | "s" | "cancel" | "c" => JobAction::Stop,
        "status" | "st" => JobAction::Status,
        other => anyhow::bail!("unknown job action: {}", other),
    };

    Ok(JobArgs { action, job_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    // `/job` 和 `/j` 共用同一套英文动作参数。
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
            parse_job_args(&["/j", "r", "456"]).unwrap(),
            JobArgs {
                action: JobAction::Resume,
                job_id: 456,
            }
        );
        assert_eq!(
            parse_job_args(&["/j", "s", "789"]).unwrap(),
            JobArgs {
                action: JobAction::Stop,
                job_id: 789,
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
            parse_job_args(&["/j", "c", "654"]).unwrap(),
            JobArgs {
                action: JobAction::Stop,
                job_id: 654,
            }
        );
        assert_eq!(
            parse_job_args(&["/j", "st", "42"]).unwrap(),
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
        assert!(parse_job_args(&["/j", "bad", "1"]).is_err());
        assert!(parse_job_args(&["/j", "p"]).is_err());
        assert!(parse_job_args(&["/j", "p", "abc"]).is_err());
    }
}
