// `/job` 参数解析。
// 用户输入统一使用长动作参数；callback payload 仍保持短格式以压缩长度。

/// 任务控制动作类型枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum JobAction {
    /// 暂停正在进行的下载或上传任务
    Pause,
    /// 唤醒并恢复已暂停的任务
    Resume,
    /// 彻底终止并停止任务
    Stop,
    /// 查询并展示任务详细执行进度与状态
    Status,
}

/// `/job` 命令行参数解析结果结构体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct JobArgs {
    /// 请求执行的目标控制动作
    pub(super) action: JobAction,
    /// 目标任务的主键 ID
    pub(super) job_id: i64,
}

/// `/job` 内联按钮回调动作枚举。
///
/// callback 只承载轻量控制，不放链接和长文本，避免 Telegram payload 过长。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum JobCallbackAction {
    /// 回调动作：暂停任务
    Pause,
    /// 回调动作：唤醒恢复任务
    Resume,
    /// 回调动作：停止二次确认或直接请求停止
    StopConfirm,
    /// 回调动作：真正执行停止
    Stop,
    /// 回调动作：刷新任务详情卡片
    Status,
}

/// `/job` 回调数据解析结果结构体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct JobCallbackArgs {
    /// 回调中所请求的动作类型
    pub(super) action: JobCallbackAction,
    /// 目标任务 ID
    pub(super) job_id: i64,
}

/// `/job` callback payload 的统一协议前缀。
const JOB_CALLBACK_PREFIX: &str = "j:";

/// 解析 `/job <action> <job_id>` 命令行参数切片。
///
/// # 参数
/// - `text`: 命令行按空格分词后的切片
pub(super) fn parse_job_args(text: &[&str]) -> anyhow::Result<JobArgs> {
    // 提取第 1 个参数作为 action
    let action = text
        .get(1)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("usage: /job <pause|resume|stop|status> <job_id>"))?;
    // 提取第 2 个参数并解析为数字 job_id
    let job_id = text
        .get(2)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("usage: /job <pause|resume|stop|status> <job_id>"))?
        .parse::<i64>()?;

    // 匹配映射到合法动作枚举
    let action = match action {
        "pause" => JobAction::Pause,
        "resume" => JobAction::Resume,
        "stop" | "cancel" => JobAction::Stop,
        "status" => JobAction::Status,
        other => anyhow::bail!("unknown job action: {other}"),
    };

    Ok(JobArgs { action, job_id })
}

/// 判断给定的 callback payload 字符串是否属于 `/job` 业务线。
pub(super) fn is_job_callback_data(data: &str) -> bool {
    data.starts_with(JOB_CALLBACK_PREFIX)
}

/// 构造 `/job` callback payload 字符串。
///
/// payload 采用 `j:<action>:<job_id>`，短格式便于后续继续加按钮。
///
/// # 参数
/// - `action`: 任务回调动作枚举
/// - `job_id`: 目标任务 ID
pub(super) fn build_job_callback_data(action: JobCallbackAction, job_id: i64) -> String {
    let action = match action {
        JobCallbackAction::Pause => "p",
        JobCallbackAction::Resume => "r",
        JobCallbackAction::StopConfirm => "sc",
        JobCallbackAction::Stop => "s",
        JobCallbackAction::Status => "st",
    };
    format!("{JOB_CALLBACK_PREFIX}{action}:{job_id}")
}

/// 解析 `/job` callback payload 字符串，还原为结构化参数。
///
/// # 参数
/// - `data`: 原始回调载荷
pub(super) fn parse_job_callback_data(data: &str) -> Option<JobCallbackArgs> {
    // 剥离 "j:" 前缀
    let payload = data.strip_prefix(JOB_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    // 解析短缩写动作
    let action = match parts.next()? {
        "p" => JobCallbackAction::Pause,
        "r" => JobCallbackAction::Resume,
        "sc" => JobCallbackAction::StopConfirm,
        "s" => JobCallbackAction::Stop,
        "st" => JobCallbackAction::Status,
        _ => return None,
    };
    // 解析任务 ID
    let job_id = parts.next()?.parse::<i64>().ok()?;
    // 校验不能有多余字段
    if parts.next().is_some() {
        return None;
    }
    Some(JobCallbackArgs { action, job_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证 `/job` 只接受长动作参数；短格式只保留在 callback payload 内部。
    #[test]
    fn test_parse_job_args() {
        // 暂停命令
        assert_eq!(
            parse_job_args(&["/job", "pause", "123"]).unwrap(),
            JobArgs {
                action: JobAction::Pause,
                job_id: 123,
            }
        );
        // 取消/停止命令
        assert_eq!(
            parse_job_args(&["/job", "cancel", "321"]).unwrap(),
            JobArgs {
                action: JobAction::Stop,
                job_id: 321,
            }
        );
        // 恢复命令
        assert_eq!(
            parse_job_args(&["/job", "resume", "654"]).unwrap(),
            JobArgs {
                action: JobAction::Resume,
                job_id: 654,
            }
        );
        // 状态详情查询命令
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
        // 非法动作应报错
        assert!(parse_job_args(&["/job", "bad", "1"]).is_err());
        // 缺少 ID 参数应报错
        assert!(parse_job_args(&["/job", "pause"]).is_err());
        // 非数字 ID 应报错
        assert!(parse_job_args(&["/job", "pause", "abc"]).is_err());
    }

    /// 验证 callback payload 序列化与反序列化的往返准确性。
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

        // 非法前缀与错误格式校验
        assert_eq!(parse_job_callback_data("d:r:run:8:1"), None);
        assert_eq!(parse_job_callback_data("j:x:42"), None);
        assert_eq!(parse_job_callback_data("j:st:not-int"), None);
    }
}
