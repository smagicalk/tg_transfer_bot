// TDLib 错误适配层：
// 将 `tdlib_rs::types::Error` 适配为标准 Error，便于 anyhow 统一处理。
use std::fmt;

use crate::tgbot::send::{ReplyPanel, build_callback_button, build_copy_button};
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::{self as transfer_mod};
use crate::tgbot::transfer::{
    build_help_message_button_data, build_menu_home_button_data_for_outer,
    build_menu_new_transfer_button_data_for_outer, build_view_commands_button,
};

#[derive(Debug)]
pub struct TdError(pub tdlib_rs::types::Error);

impl fmt::Display for TdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code={}, message={}", self.0.code, self.0.message)
    }
}

impl std::error::Error for TdError {}

/// 命令错误的用户提示。
///
/// `tgbot.rs` 只负责更新分发，错误说明和下一步建议放在这里统一维护。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CommandErrorHint {
    pub(crate) title: &'static str,
    pub(crate) status: &'static str,
    pub(crate) reason: &'static str,
    pub(crate) advice: &'static str,
    pub(crate) primary_label: &'static str,
    pub(crate) primary_command: &'static str,
    pub(crate) primary_action: CommandErrorPrimaryAction,
    pub(crate) help_command: &'static str,
}

/// 命令错误卡片主按钮的真实行为。
///
/// 之前只存文案和命令字符串，容易出现“按钮写的是查看余额，实际却跳菜单”的错位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandErrorPrimaryAction {
    OpenHelp,
    OpenMenu,
    StartTransfer,
}

/// 命令错误分类到“下一步操作”的映射。
///
/// 错误原因由 transfer 共享分类提供；这里只决定给用户复制哪个命令最有效。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CommandErrorAction {
    primary_label: &'static str,
    primary_command: &'static str,
    primary_action: CommandErrorPrimaryAction,
    help_command: &'static str,
}

/// 回复未知命令，避免用户输入错误时只在日志里可见。
pub(crate) async fn send_unknown_command_message(
    _command: &str,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    ReplyPanel::card(
        [
            "未知命令".to_owned(),
            format!("状态：{}", card::code("invalid-command")),
            card::DIVIDER.to_owned(),
            card::section("下一步"),
            card::note("日常操作请使用交互菜单；需要命令时点击“查看命令”。"),
        ]
        .join("\n"),
    )
    .row(vec![
        build_view_commands_button(None),
        build_callback_button(
            "打开菜单",
            &build_menu_home_button_data_for_outer(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
    ])
    .send(chat_id, client_id)
    .await
}

/// 回复命令执行错误。
///
/// 命令处理失败大多是参数错误或当前任务状态不允许操作；这里给用户明确反馈，
/// 同时保留可复制错误详情，避免问题只出现在日志中。
pub(crate) async fn send_command_error_message(
    command: &str,
    err: &anyhow::Error,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let error_text = format!("{err:#}");
    let hint = command_error_hint(&error_text);
    let result = ReplyPanel::card(
        [
            hint.title.to_owned(),
            format!("状态：{}", card::code(hint.status)),
            card::DIVIDER.to_owned(),
            card::section("原因"),
            hint.reason.to_owned(),
            card::section("建议"),
            hint.advice.to_owned(),
            card::section("错误"),
            card::pre_code(&error_text),
            card::section("下一步"),
            card::note("优先使用下方按钮继续操作；命令说明仅在点击后显示。"),
        ]
        .join("\n"),
    )
    .row(vec![
        build_primary_action_button(&hint),
        build_view_commands_button(help_topic_from_command(hint.help_command)),
        build_copy_button(
            "复制错误",
            &error_text,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(chat_id, client_id)
    .await;
    if let Err(send_err) = &result {
        tracing::warn!(
            chat_id,
            client_id,
            command,
            error = %send_err,
            "send command error card failed"
        );
    }
    result
}

/// 根据错误文本选择更可执行的用户提示。
pub(crate) fn command_error_hint(error_text: &str) -> CommandErrorHint {
    let hint = transfer_mod::classify_transfer_error_text(error_text);
    let action = command_error_action(hint.kind);
    CommandErrorHint {
        title: hint.title,
        status: hint.status,
        reason: hint.reason,
        advice: hint.advice,
        primary_label: action.primary_label,
        primary_command: action.primary_command,
        primary_action: action.primary_action,
        help_command: action.help_command,
    }
}

/// 命令错误分类到“下一步操作”的映射。
fn command_error_action(kind: transfer_mod::TransferErrorKind) -> CommandErrorAction {
    match kind {
        transfer_mod::TransferErrorKind::MissingTarget => CommandErrorAction {
            primary_label: "选择目标转存",
            primary_command: "/transfer",
            primary_action: CommandErrorPrimaryAction::StartTransfer,
            help_command: "/help transfer",
        },
        transfer_mod::TransferErrorKind::InvalidArgs => CommandErrorAction {
            primary_label: "查看转存命令",
            primary_command: "/help transfer",
            primary_action: CommandErrorPrimaryAction::OpenHelp,
            help_command: "/help",
        },
        transfer_mod::TransferErrorKind::SourceDenied
        | transfer_mod::TransferErrorKind::PermissionDenied => CommandErrorAction {
            primary_label: "打开菜单",
            primary_command: "/menu",
            primary_action: CommandErrorPrimaryAction::OpenMenu,
            help_command: "/help transfer",
        },
        _ => CommandErrorAction {
            primary_label: "查看命令",
            primary_command: "/help",
            primary_action: CommandErrorPrimaryAction::OpenHelp,
            help_command: "/help",
        },
    }
}

/// 根据错误提示定义构造真正的主按钮。
fn build_primary_action_button(hint: &CommandErrorHint) -> tdlib_rs::types::InlineKeyboardButton {
    match hint.primary_action {
        CommandErrorPrimaryAction::OpenHelp => build_callback_button(
            hint.primary_label,
            &build_help_message_button_data(help_topic_from_command(hint.primary_command)),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        CommandErrorPrimaryAction::OpenMenu => build_callback_button(
            hint.primary_label,
            &build_menu_home_button_data_for_outer(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        CommandErrorPrimaryAction::StartTransfer => build_callback_button(
            hint.primary_label,
            &build_menu_new_transfer_button_data_for_outer(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
    }
}

/// 从 `/help <topic>` 中提取帮助主题；`/help` 本身对应命令目录。
fn help_topic_from_command(command: &str) -> Option<&str> {
    command
        .strip_prefix("/help")
        .map(str::trim)
        .filter(|topic| !topic.is_empty())
}

/// 自动转存媒体失败时给出可执行提示。
///
/// 最常见原因是当前请求 chat 没配置默认 target；引导用户回到交互式目标选择。
pub(crate) async fn send_auto_transfer_hint_message(
    err: &anyhow::Error,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let error_text = format!("{err:#}");
    if error_text.contains("无法定位原始消息") {
        return ReplyPanel::card(
            [
                "无法识别转发来源".to_owned(),
                format!("状态：{}", card::code("source-unavailable")),
                card::DIVIDER.to_owned(),
                card::section("说明"),
                "这条转发消息没有可稳定还原的原始 message_id，不能安全地作为转存源。".to_owned(),
                card::section("建议"),
                "请改用原始消息链接，或重新开始转存后发送一条 bot 可见媒体。".to_owned(),
            ]
            .join("\n"),
        )
        .row(vec![
            build_callback_button(
                "开始转存",
                &build_menu_new_transfer_button_data_for_outer(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            build_view_commands_button(Some("transfer")),
        ])
        .row(vec![build_callback_button(
            "打开菜单",
            &build_menu_home_button_data_for_outer(),
            tdlib_rs::enums::ButtonStyle::Default,
        )])
        .send(chat_id, client_id)
        .await;
    }

    ReplyPanel::card(
        [
            "自动转存未启动".to_owned(),
            format!("状态：{}", card::code("need-target")),
            card::DIVIDER.to_owned(),
            card::section("原因"),
            card::pre_code(error_text),
            card::section("下一步"),
            "点击“开始转存”，按提示重新选择来源和目标。".to_owned(),
        ]
        .join("\n"),
    )
    .row(vec![
        build_callback_button(
            "开始转存",
            &build_menu_new_transfer_button_data_for_outer(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_view_commands_button(Some("transfer")),
        build_callback_button(
            "打开菜单",
            &build_menu_home_button_data_for_outer(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(chat_id, client_id)
    .await
}
