// `/menu` ForceReply 输入状态。
// 这里只保存“正在填写命令”的临时草稿，真实任务状态仍全部落数据库。

use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::{lookup, transfer_cmd};
use super::text::build_transfer_prompt_text;

/// 菜单输入草稿超时时间。
const INPUT_TTL: Duration = Duration::from_secs(10 * 60);

/// 输入草稿索引。
///
/// 同一个管理员可能在多个管理 chat 中操作，因此用 `(chat_id, user_id)` 做隔离。
type DraftKey = (i64, i64);

/// 全局输入草稿表。
static MENU_INPUT_DRAFTS: LazyLock<std::sync::Mutex<HashMap<DraftKey, MenuInputDraft>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// 菜单输入流程。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuInputKind {
    Transfer,
    TransferDefault,
    Lookup,
    LookupDefault,
}

impl MenuInputKind {
    /// 是否只需要源链接，目标 chat 交给配置默认值解析。
    fn uses_default_target(self) -> bool {
        matches!(self, Self::TransferDefault | Self::LookupDefault)
    }

    /// 归一化到实际命令类型。
    fn command_kind(self) -> Self {
        match self {
            Self::Transfer | Self::TransferDefault => Self::Transfer,
            Self::Lookup | Self::LookupDefault => Self::Lookup,
        }
    }

    /// 当前流程的短命令名。
    fn command_name(self) -> &'static str {
        match self.command_kind() {
            Self::Transfer => "/t",
            Self::Lookup => "/lk",
            Self::TransferDefault | Self::LookupDefault => unreachable!("kind is normalized"),
        }
    }

    /// 源链接输入标题。
    pub(super) fn source_title(self) -> &'static str {
        match self {
            Self::Transfer => "转存源链接",
            Self::TransferDefault => "快速转存",
            Self::Lookup => "查询源链接",
            Self::LookupDefault => "快速查询",
        }
    }

    /// 源链接输入说明。
    pub(super) fn source_detail(self) -> &'static str {
        match self {
            Self::Transfer => "请回复要转存的 Telegram 消息或相册链接。",
            Self::TransferDefault => "请回复源链接，目标 chat 将使用配置默认值。",
            Self::Lookup => "请回复要查询的 Telegram 消息或相册链接。",
            Self::LookupDefault => "请回复源链接，目标 chat 将使用配置默认值。",
        }
    }

    /// 日志中使用的输入流程名，避免直接打印 Debug 后未来重命名影响排查关键词。
    fn log_name(self) -> &'static str {
        match self {
            Self::Transfer => "transfer",
            Self::TransferDefault => "transfer_default",
            Self::Lookup => "lookup",
            Self::LookupDefault => "lookup_default",
        }
    }
}

/// 菜单输入阶段。
#[derive(Debug, Clone, PartialEq, Eq)]
enum MenuInputStep {
    SourceLink {
        kind: MenuInputKind,
    },
    TargetChat {
        kind: MenuInputKind,
        source_link: String,
    },
}

/// 菜单输入草稿。
#[derive(Debug, Clone)]
struct MenuInputDraft {
    step: MenuInputStep,
    updated_at: Instant,
}

/// 取草稿的结果。
#[derive(Debug, Clone)]
enum DraftTakeResult {
    None,
    Active(MenuInputDraft),
    Expired,
}

/// 开始一个菜单输入流程。
pub(super) fn start_menu_input(chat_id: i64, user_id: i64, kind: MenuInputKind) {
    let mut drafts = MENU_INPUT_DRAFTS
        .lock()
        .expect("menu input draft mutex poisoned");
    purge_expired_locked(&mut drafts);
    drafts.insert(
        (chat_id, user_id),
        MenuInputDraft {
            step: MenuInputStep::SourceLink { kind },
            updated_at: Instant::now(),
        },
    );
    tracing::debug!(
        chat_id,
        user_id,
        input_kind = kind.log_name(),
        "menu input draft started"
    );
}

/// 取消一个菜单输入流程。
pub(super) fn cancel_menu_input(chat_id: i64, user_id: i64) -> bool {
    let mut drafts = MENU_INPUT_DRAFTS
        .lock()
        .expect("menu input draft mutex poisoned");
    let removed = drafts.remove(&(chat_id, user_id)).is_some();
    if removed {
        tracing::debug!(chat_id, user_id, "menu input draft cancelled");
    }
    removed
}

/// 处理菜单输入。
///
/// 返回 true 表示本条消息已被输入流程消费；返回 false 表示没有匹配草稿。
pub(super) async fn handle_menu_input(
    text: &str,
    config: std::sync::Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    let input = text.trim();
    if input.is_empty() {
        return Ok(false);
    }

    let key = (request_chat_id, sender_user_id);
    let draft = match take_current_draft(key) {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input draft expired"
            );
            send::ReplyPanel::card(build_transfer_prompt_text(
                "输入已过期",
                "上一次菜单输入已超过 10 分钟，请重新打开 /m。",
            ))
            .row(vec![send::build_copy_button(
                "复制 /m",
                "/m",
                tdlib_rs::enums::ButtonStyle::Primary,
            )])
            .send(request_chat_id, client_id)
            .await?;
            return Ok(true);
        }
        DraftTakeResult::None => {
            tracing::trace!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input draft not found"
            );
            return Ok(false);
        }
    };

    match draft.step {
        MenuInputStep::SourceLink { kind } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                "menu input source link received"
            );
            if !looks_like_telegram_link(input) {
                put_draft(
                    key,
                    MenuInputDraft {
                        step: MenuInputStep::SourceLink { kind },
                        updated_at: Instant::now(),
                    },
                );
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    input_kind = kind.log_name(),
                    "menu input source link rejected"
                );
                send::send_card_message_with_force_reply_returning(
                    build_transfer_prompt_text(
                        "源链接格式不正确",
                        "请回复 t.me 消息链接，或发送 /cancel 取消。",
                    ),
                    request_chat_id,
                    "输入 https://t.me/... 链接",
                    client_id,
                )
                .await?;
                return Ok(true);
            }

            if kind.uses_default_target() {
                let Some(target_chat_id) = resolve_default_target(&config, request_chat_id) else {
                    put_draft(
                        key,
                        MenuInputDraft {
                            step: MenuInputStep::TargetChat {
                                kind: kind.command_kind(),
                                source_link: input.to_owned(),
                            },
                            updated_at: Instant::now(),
                        },
                    );
                    tracing::debug!(
                        request_chat_id,
                        sender_user_id,
                        request_message_id,
                        input_kind = kind.log_name(),
                        "menu input default target missing, asking target chat"
                    );
                    send::send_card_message_with_force_reply_returning(
                        build_transfer_prompt_text(
                            "缺少默认目标",
                            "配置里没有当前 chat 的默认目标，请回复目标 chat_id。",
                        ),
                        request_chat_id,
                        "输入目标 chat_id",
                        client_id,
                    )
                    .await?;
                    return Ok(true);
                };
                let command_owned = vec![
                    kind.command_name().to_owned(),
                    input.to_owned(),
                    target_chat_id.to_string(),
                ];
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    target_chat_id,
                    input_kind = kind.log_name(),
                    "menu input resolved default target"
                );
                run_existing_command(
                    kind,
                    command_owned,
                    config,
                    request_chat_id,
                    request_message_id,
                    client_id,
                )
                .await?;
                return Ok(true);
            }

            put_draft(
                key,
                MenuInputDraft {
                    step: MenuInputStep::TargetChat {
                        kind,
                        source_link: input.to_owned(),
                    },
                    updated_at: Instant::now(),
                },
            );
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                "menu input asking target chat"
            );
            send::send_card_message_with_force_reply_returning(
                build_transfer_prompt_text(
                    "目标 chat",
                    "请回复目标 chat_id；如果配置了默认目标，也可以回复 default。",
                ),
                request_chat_id,
                "输入目标 chat_id 或 default",
                client_id,
            )
            .await?;
            Ok(true)
        }
        MenuInputStep::TargetChat { kind, source_link } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                "menu input target chat received"
            );
            let target_arg = if input.eq_ignore_ascii_case("default") {
                None
            } else if input.parse::<i64>().is_ok() {
                Some(input.to_owned())
            } else {
                put_draft(
                    key,
                    MenuInputDraft {
                        step: MenuInputStep::TargetChat { kind, source_link },
                        updated_at: Instant::now(),
                    },
                );
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    input_kind = kind.log_name(),
                    "menu input target chat rejected"
                );
                send::send_card_message_with_force_reply_returning(
                    build_transfer_prompt_text(
                        "目标 chat 格式不正确",
                        "请回复数字 chat_id，或回复 default 使用配置默认目标。",
                    ),
                    request_chat_id,
                    "输入目标 chat_id 或 default",
                    client_id,
                )
                .await?;
                return Ok(true);
            };

            let mut command_owned = vec![kind.command_name().to_owned(), source_link];
            if let Some(target_arg) = target_arg {
                command_owned.push(target_arg);
            }
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                target_is_default = command_owned.len() == 2,
                "menu input completed, dispatching command"
            );
            run_existing_command(
                kind,
                command_owned,
                config,
                request_chat_id,
                request_message_id,
                client_id,
            )
            .await?;
            Ok(true)
        }
    }
}

/// 调用已有命令入口，避免菜单输入流复制转存/查询业务逻辑。
async fn run_existing_command(
    kind: MenuInputKind,
    command_owned: Vec<String>,
    config: std::sync::Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    match kind.command_kind() {
        MenuInputKind::Transfer => {
            transfer_cmd::transfer_command(
                command_refs,
                config,
                request_chat_id,
                request_message_id,
                client_id,
            )
            .await
        }
        MenuInputKind::Lookup => {
            lookup::lookup_command(command_refs, config, request_chat_id, client_id).await
        }
        MenuInputKind::TransferDefault | MenuInputKind::LookupDefault => {
            unreachable!("kind is normalized")
        }
    }
}

/// 取出当前草稿；若草稿过期，则清理后返回 None。
fn take_current_draft(key: DraftKey) -> DraftTakeResult {
    let mut drafts = MENU_INPUT_DRAFTS
        .lock()
        .expect("menu input draft mutex poisoned");
    let Some(draft) = drafts.remove(&key) else {
        purge_expired_locked(&mut drafts);
        return DraftTakeResult::None;
    };
    if Instant::now().duration_since(draft.updated_at) > INPUT_TTL {
        purge_expired_locked(&mut drafts);
        return DraftTakeResult::Expired;
    }
    purge_expired_locked(&mut drafts);
    DraftTakeResult::Active(draft)
}

/// 写回草稿。
fn put_draft(key: DraftKey, draft: MenuInputDraft) {
    let mut drafts = MENU_INPUT_DRAFTS
        .lock()
        .expect("menu input draft mutex poisoned");
    drafts.insert(key, draft);
}

/// 清理超时草稿。
fn purge_expired_locked(drafts: &mut HashMap<DraftKey, MenuInputDraft>) {
    let now = Instant::now();
    drafts.retain(|_, draft| now.duration_since(draft.updated_at) <= INPUT_TTL);
}

/// 粗略判断是否是 Telegram 消息链接。
///
/// 真正合法性仍由 spider 层解析；这里仅避免明显错误输入推进到下一步。
fn looks_like_telegram_link(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
}

/// 解析菜单“快速转存/查询”使用的默认目标。
///
/// 这里提前解析是为了在缺少默认目标时继续引导输入目标，而不是让复用的命令入口直接报错。
fn resolve_default_target(config: &BotConfig, request_chat_id: i64) -> Option<i64> {
    config
        .target_map
        .get(&request_chat_id)
        .copied()
        .or_else(|| config.target_map.get(&0).copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Telegram 链接预检查只做粗筛，最终解析仍由 spider 负责。
    #[test]
    fn test_looks_like_telegram_link() {
        assert!(looks_like_telegram_link("https://t.me/c/1/2"));
        assert!(looks_like_telegram_link("t.me/c/1/2"));
        assert!(!looks_like_telegram_link("https://example.com"));
    }

    // 草稿应按 chat + user 隔离，避免多个管理员互相覆盖输入。
    #[test]
    fn test_start_and_cancel_menu_input() {
        start_menu_input(1, 2, MenuInputKind::Transfer);
        assert!(cancel_menu_input(1, 2));
        assert!(!cancel_menu_input(1, 2));
    }

    // 不同输入流程应使用对应的短命令，最终复用已有命令入口。
    #[test]
    fn test_menu_input_kind_command_name() {
        assert_eq!(MenuInputKind::Transfer.command_name(), "/t");
        assert_eq!(MenuInputKind::TransferDefault.command_name(), "/t");
        assert_eq!(MenuInputKind::Lookup.command_name(), "/lk");
        assert_eq!(MenuInputKind::LookupDefault.command_name(), "/lk");
    }

    // 快速转存应优先使用当前请求 chat 的默认目标，再使用全局兜底目标。
    #[test]
    fn test_resolve_default_target() {
        let mut config = BotConfig::default();
        assert_eq!(resolve_default_target(&config, 1), None);

        config.target_map.insert(0, -100);
        assert_eq!(resolve_default_target(&config, 1), Some(-100));

        config.target_map.insert(1, -200);
        assert_eq!(resolve_default_target(&config, 1), Some(-200));
    }
}
