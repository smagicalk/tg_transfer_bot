// 转存失败回复卡片。
// 正文使用统一卡片风格，错误详情单独用等宽代码块，兼顾美观和排查便利。

use super::super::card;
use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};
use super::super::command::{
    build_job_status_button_data, build_lookup_retry_transfer_button_data,
};

/// 转存错误的稳定分类。
///
/// TDLib 和 anyhow 的错误类型不稳定，外层展示不能依赖具体 error type；
/// 用文本关键词做保守分类，保证命令错误卡片和后台失败卡片的解释口径一致。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot) enum TransferErrorKind {
    TdlibRequest,
    MissingTarget,
    SourceDenied,
    PermissionDenied,
    InvalidArgs,
    AlbumUnsupported,
    DownloadFailed,
    UploadFailed,
    Unknown,
}

/// 转存错误的用户提示。
///
/// `title/status` 适合卡片摘要，`reason/advice` 适合正文说明；按钮和命令入口由调用方决定。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot) struct TransferErrorHint {
    pub kind: TransferErrorKind,
    pub title: &'static str,
    pub status: &'static str,
    pub reason: &'static str,
    pub advice: &'static str,
}

/// 根据错误文本选择用户可执行的提示。
///
/// 顺序很重要：先匹配 TDLib 请求解析、目标等明确错误，再匹配较宽泛的权限和下载关键词。
pub(in crate::tgbot) fn classify_transfer_error_text(error_text: &str) -> TransferErrorHint {
    let lower = error_text.to_ascii_lowercase();

    if contains_any(
        &lower,
        &[
            "failed to parse json object as tdlib request",
            "wrong padding",
        ],
    ) {
        return transfer_error_hint(
            TransferErrorKind::TdlibRequest,
            "TDLib 请求错误",
            "tdlib-request",
            "TDLib 请求或按钮回调数据解析失败。",
            "这通常不是源链接问题；请复制错误详情查看日志，重新打开菜单后再试。",
        );
    }

    if lower.contains("not found transfer target") {
        return transfer_error_hint(
            TransferErrorKind::MissingTarget,
            "缺少目标",
            "need-target",
            "当前请求 chat 没有匹配到默认目标，也没有在命令里指定目标。",
            "重新发送转存命令并带上目标 chat_id 或别名，或者在目标配置中设置默认目标。",
        );
    }

    if contains_any(&lower, &["unsupported link type", "usage:"]) {
        return transfer_error_hint(
            TransferErrorKind::InvalidArgs,
            "参数格式错误",
            "invalid-args",
            "源链接格式或命令参数不被当前流程支持。",
            "请使用 Telegram 消息链接，例如 https://t.me/c/.../...，或直接把源消息转发给 bot。",
        );
    }

    if contains_any(
        &lower,
        &[
            "message link info doesn't contain message",
            "message not found",
        ],
    ) {
        return transfer_error_hint(
            TransferErrorKind::SourceDenied,
            "源不可访问",
            "source-denied",
            "源消息不存在、已删除，或当前读取账号看不到这条消息。",
            "请转发源消息给 bot，或确保 bot/备用 user 至少有一个账号能访问源聊天。",
        );
    }

    if contains_any(
        &lower,
        &[
            "not a member",
            "have no access",
            "can't access",
            "forbidden",
            "unauthorized",
            "chat not found",
        ],
    ) {
        return transfer_error_hint(
            TransferErrorKind::PermissionDenied,
            "权限不足",
            "permission-denied",
            "读取源聊天或写入目标聊天的权限不足。",
            "确认 bot 在目标聊天有发消息权限；链接源如果 bot 不可见，备用 user 必须加入源群或频道。",
        );
    }

    if contains_any(
        &lower,
        &[
            "album upload doesn't support",
            "document album requires",
            "audio album requires",
        ],
    ) {
        return transfer_error_hint(
            TransferErrorKind::AlbumUnsupported,
            "Album 规则限制",
            "album-limited",
            "这组消息不能按 Telegram album 规则合并发送。",
            "多条文本、语音、GIF 或混合 document/audio 当前不会自动拆成单条发送；请拆分来源后重试。",
        );
    }

    if contains_any(
        &lower,
        &[
            "download",
            "file missing",
            "empty local path",
            "failed during prepare",
            "prepare upload content failed",
        ],
    ) {
        return transfer_error_hint(
            TransferErrorKind::DownloadFailed,
            "文件准备失败",
            "download-failed",
            "文件下载或上传内容准备失败。",
            "确认账号能访问源文件、本地磁盘空间足够；如果是 bot 读取失败，系统会尝试切 user 读取。",
        );
    }

    if contains_any(
        &lower,
        &["send_message", "upload failed", "failed during upload"],
    ) {
        return transfer_error_hint(
            TransferErrorKind::UploadFailed,
            "上传失败",
            "upload-failed",
            "文件已准备好，但发送到目标聊天失败。",
            "确认上传账号在目标聊天有发媒体权限，且目标没有限制文件类型或大小。",
        );
    }

    transfer_error_hint(
        TransferErrorKind::Unknown,
        "命令执行失败",
        "failed",
        "暂未匹配到明确分类。",
        "请复制错误详情查看日志；确认源链接、目标 chat、登录状态和权限后重试。",
    )
}

/// 构造静态错误提示，避免每个分支重复字段名。
fn transfer_error_hint(
    kind: TransferErrorKind,
    title: &'static str,
    status: &'static str,
    reason: &'static str,
    advice: &'static str,
) -> TransferErrorHint {
    TransferErrorHint {
        kind,
        title,
        status,
        reason,
        advice,
    }
}

/// 发送失败信息。
pub(in crate::tgbot::transfer) async fn send_failure_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    err: anyhow::Error,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let retry_command = build_transfer_command(source_link, target_chat_id, CommandStyle::Long);
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Long);
    let buttons = build_failure_buttons(job_id, &build_lookup_retry_transfer_button_data());
    let sent = crate::tgbot::send::send_card_message_with_buttons_returning(
        format_failure_card_text(
            title,
            source_link,
            target_chat_id,
            job_id,
            &retry_command,
            &lookup_command,
            &err,
        ),
        notify_chat_id,
        buttons,
        client_id,
    )
    .await?;
    // 失败页点击“重新转存”时，沿用 lookup 的轻量上下文机制复用源链接与目标 chat，
    // 避免把长链接塞进 callback_data 里。
    crate::app_context::app_context().lookup_retry.put_context(
        notify_chat_id,
        notify_chat_id,
        sent.id,
        crate::app_context::LookupRetryContext {
            source_link: source_link.to_owned(),
            target_chat_id,
        },
    );
    Ok(())
}

/// 构造失败卡片按钮。
///
/// 失败详情和重试命令保留在正文；按钮区优先放能直接点击的 callback。
/// 重新转存沿用 lookup 的短 callback + 进程内上下文，避免把长链接塞进 callback_data。
fn build_failure_buttons(
    job_id: Option<i64>,
    retry_callback_data: &str,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut action_row = Vec::new();
    if let Some(job_id) = job_id {
        action_row.push(crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    let retry_style = if job_id.is_some() {
        tdlib_rs::enums::ButtonStyle::Primary
    } else {
        tdlib_rs::enums::ButtonStyle::Default
    };
    action_row.push(crate::tgbot::send::build_callback_button(
        "重新转存",
        retry_callback_data,
        retry_style,
    ));

    vec![
        action_row,
        crate::tgbot::transfer::outcome::build_list_menu_row("查看失败列表", "fail"),
    ]
}

/// 构造失败卡片正文。
///
/// 用户号模式下按钮会被发送层丢弃，因此重试、查询和失败列表命令必须出现在正文里。
pub(in crate::tgbot::transfer) fn format_failure_card_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    retry_command: &str,
    lookup_command: &str,
    err: &anyhow::Error,
) -> String {
    let mut lines = vec![
        title.to_owned(),
        card::summary_line("failed", job_id, target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("错误"),
        card::pre_code(format!("{:#}", err)),
        card::section("建议"),
    ];
    lines.extend(build_failure_advice_lines(err));
    lines.extend([
        card::section("命令"),
        card::command_line("重试", retry_command),
        card::command_line("查询", lookup_command),
        card::command_line(
            "列表",
            build_downloads_command(Some("fail"), None, None, CommandStyle::Long),
        ),
        String::new(),
    ]);
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 根据错误文本生成用户可执行的排查建议。
///
/// TDLib 错误来自远端状态和本地 client 状态，类型不稳定；这里用保守的文本分类给出下一步，
/// 但原始错误仍保留在“错误”代码块里，方便日志排查时回到真实原因。
fn build_failure_advice_lines(err: &anyhow::Error) -> Vec<String> {
    let err_text = format!("{:#}", err);
    let hint = classify_transfer_error_text(&err_text);
    vec![
        format!("原因：{}", hint.reason),
        format!("处理：{}", hint.advice),
    ]
}

/// 判断错误文本是否包含任一关键词。
fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::{
        TransferErrorKind, build_failure_advice_lines, build_failure_buttons,
        classify_transfer_error_text, format_failure_card_text,
    };

    // 恢复失败已知 job_id 时，应能从失败卡片直接跳任务详情和失败列表。
    #[test]
    fn test_build_failure_buttons_with_job_id() {
        let rows = build_failure_buttons(Some(42), "lk:rt");
        let buttons = rows.iter().flatten().collect::<Vec<_>>();

        assert_eq!(rows.len(), 2);
        assert!(buttons.iter().any(|button| button.text == "查看任务详情"));
        assert!(buttons.iter().any(|button| button.text == "重新转存"));
        assert_eq!(rows[1][0].text, "查看失败列表");
        assert_eq!(rows[1][1].text, "菜单");
        assert!(!buttons.iter().any(|button| button.text == "复制重试命令"));
        for button in buttons {
            assert!(matches!(
                button.r#type,
                tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
            ));
        }
    }

    // 失败正文应保留重试命令、查询命令和完整错误，用户号模式下也能继续操作。
    #[test]
    fn test_format_failure_card_text() {
        let err = anyhow::anyhow!("network failed");
        let text = format_failure_card_text(
            "转存失败",
            "https://t.me/c/1/2",
            -100,
            Some(42),
            "/transfer https://t.me/c/1/2 -100",
            "/lookup https://t.me/c/1/2 -100",
            &err,
        );

        assert!(text.contains("状态：‹failed›"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("«network failed»"));
        assert!(text.contains("■ 建议"));
        assert!(text.contains("确认源链接、目标 chat、登录状态和权限后重试"));
        assert!(text.contains("重试：‹/transfer https://t.me/c/1/2 -100›"));
        assert!(text.contains("查询：‹/lookup https://t.me/c/1/2 -100›"));
        assert!(text.contains("列表：‹/downloads fail›"));
    }

    // 源消息不可见时，应直接告诉用户检查源消息和账号访问权限。
    #[test]
    fn test_build_failure_advice_for_missing_source_message() {
        let advice =
            build_failure_advice_lines(&anyhow::anyhow!("code=400, message=Message not found"));

        assert!(advice.iter().any(|line| line.contains("源消息不存在")));
        assert!(advice.iter().any(|line| line.contains("备用 user")));
    }

    // album 规则失败时，应说明不是下载问题，而是 Telegram album 组合限制。
    #[test]
    fn test_build_failure_advice_for_album_kind_limit() {
        let advice = build_failure_advice_lines(&anyhow::anyhow!(
            "album upload doesn't support voice note item"
        ));

        assert!(advice.iter().any(|line| line.contains("album 规则")));
        assert!(
            advice
                .iter()
                .any(|line| line.contains("不会自动拆成单条发送"))
        );
    }

    // TDLib 请求解析失败通常是请求/按钮数据问题，不应被提示成源链接格式错误。
    #[test]
    fn test_build_failure_advice_for_tdlib_request_parse_error() {
        let advice = build_failure_advice_lines(&anyhow::anyhow!(
            "code=400, message=Failed to parse JSON object as TDLib request: Wrong padding length"
        ));

        assert!(advice.iter().any(|line| line.contains("TDLib 请求")));
        assert!(!advice.iter().any(|line| line.contains("源链接格式")));
    }

    // 共享分类应识别缺少目标，便于命令入口给出明确下一步。
    #[test]
    fn test_classify_transfer_error_for_missing_target() {
        let hint = classify_transfer_error_text("not found transfer target for chat 1");

        assert_eq!(hint.kind, TransferErrorKind::MissingTarget);
        assert_eq!(hint.title, "缺少目标");
    }
}
