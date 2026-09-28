//! TDLib 错误适配与命令执行错误交互卡片模块。
//!
//! # 核心职责
//! 1. **TDLib 错误封装**：将底层 `tdlib_rs::types::Error` 封装为实现 `std::error::Error` 的 `TdError`，便于与 `anyhow::Result` 无缝集成。
//! 2. **友好错误卡片渲染**：将系统/底层错误分类并转换为面向用户的结构化提示卡片（包含原因、建议、原始错误代码及交互式恢复按钮）。
//! 3. **交互动作引导**：根据错误类别自动提供最直接的恢复动作（如直接打开交互菜单、开始转存或跳转命令帮助）。

use std::fmt;

use crate::tgbot::send::{ReplyPanel, build_callback_button, build_copy_button};
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::{self as transfer_mod};
use crate::tgbot::transfer::{
    build_help_message_button_data, build_menu_home_button_data_for_outer,
    build_menu_new_transfer_button_data_for_outer, build_view_commands_button,
};

/// TDLib 返回的原生错误包装类型。
///
/// 实现了 `std::error::Error` 与 `Display`，输出格式为 `code={code}, message={message}`。
#[derive(Debug)]
pub struct TdError(pub tdlib_rs::types::Error);

impl fmt::Display for TdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code={}, message={}", self.0.code, self.0.message)
    }
}

impl std::error::Error for TdError {}

/// 命令错误的用户友好提示结构体。
///
/// `tgbot.rs` 只负责分发与路由，错误文案说明和下一步操作建议在此处集中定义。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CommandErrorHint {
    /// 提示卡片标题（例如 "参数无效"、"源不可访问"）
    pub(crate) title: &'static str,
    /// 状态标签代码（例如 "invalid-argument"、"source-unavailable"）
    pub(crate) status: &'static str,
    /// 错误原因说明
    pub(crate) reason: &'static str,
    /// 面向用户的排查或操作建议
    pub(crate) advice: &'static str,
    /// 卡片主动作按钮的展示文本
    pub(crate) primary_label: &'static str,
    /// 主操作关联的命令模板（例如 "/transfer"）
    pub(crate) primary_command: &'static str,
    /// 主按钮的实际交互行为类型
    pub(crate) primary_action: CommandErrorPrimaryAction,
    /// 帮助命令字符串（例如 "/help transfer"）
    pub(crate) help_command: &'static str,
}

/// 命令错误卡片主按钮的实际交互行为枚举。
///
/// 显式区分动作类型，避免按钮文本与实际回调逻辑不一致。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandErrorPrimaryAction {
    /// 打开帮助主题卡片
    OpenHelp,
    /// 打开主菜单
    OpenMenu,
    /// 发起新的转存流程
    StartTransfer,
}

/// 命令错误分类到下一步动作配置的内部映射结构。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CommandErrorAction {
    /// 主按钮展示文案
    primary_label: &'static str,
    /// 主命令字符串
    primary_command: &'static str,
    /// 主动作枚举
    primary_action: CommandErrorPrimaryAction,
    /// 关联的帮助命令
    help_command: &'static str,
}

/// 回复未知命令错误卡片。
///
/// 避免用户输入错误命令时机器人没有任何响应，并引导用户使用交互菜单或查看帮助。
///
/// # 参数
/// * `_command` - 用户输入的未知命令文本
/// * `chat_id` - 目标会话标识
/// * `client_id` - 响应使用的 Bot 客户端标识
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

/// 回复命令执行错误卡片。
///
/// 命令处理失败大多是参数错误、权限不足或任务当前状态不允许操作；
/// 给用户提供明确结构化反馈，同时提供一键复制错误详情按钮方便排查。
///
/// # 参数
/// * `command` - 触发错误的命令名
/// * `err` - 错误对象
/// * `chat_id` - 目标会话标识
/// * `client_id` - 响应使用的 Bot 客户端标识
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

/// 根据错误文本匹配并生成最契合的用户提示与动作配置。
///
/// # 参数
/// * `error_text` - 错误字符串
///
/// # 返回
/// 组装好的 `CommandErrorHint` 提示对象
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

/// 将转存错误类别映射到推荐的下一步操作按钮配置。
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

/// 根据 `CommandErrorHint` 中的主操作定义构建对应的内联键盘按钮。
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

/// 从 `/help <topic>` 命令字符串中解析出帮助主题参数；若仅为 `/help` 则返回 `None`。
fn help_topic_from_command(command: &str) -> Option<&str> {
    command
        .strip_prefix("/help")
        .map(str::trim)
        .filter(|topic| !topic.is_empty())
}

/// 自动转存直接发送/转发的媒体消息失败时发送针对性的可执行卡片提示。
///
/// 最常见原因是当前对话尚未配置默认转存目标；引导用户点击按钮直接进行目标选择。
///
/// # 参数
/// * `err` - 错误对象
/// * `chat_id` - 目标会话标识
/// * `client_id` - 响应使用的 Bot 客户端标识
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
