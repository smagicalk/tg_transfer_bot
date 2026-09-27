//! `/menu` 菜单中的管理员配置交互单步输入解析与分发模块。
//!
//! 负责将用户通过 ForceReply 或原生选聊提交的文本/会话信息，
//! 转换为等价的底层 `/targets` 或 `/config` 命令行参数向量并复用既有逻辑执行。

use crate::tgbot::transfer::command::config_cmd::config_field_spec_for_admin_action;
use crate::tgbot::transfer::command::targets::targets_input_spec_for_admin_action;
use crate::tgbot::transfer::command::{config_cmd, targets};

use super::state::AdminInputAction;

/// 管理输入最终映射分发的底层命令模块分类枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdminCommandKind {
    /// 目标会话管理命令（`/targets`）
    Targets,
    /// 系统运行时参数配置命令（`/config`）
    Config,
}

/// 根据管理员输入动作类型反查其归属的底层命令类别。
///
/// 避免主输入流程硬编码分支，实现统一的动作与目标模块映射。
///
/// # 参数说明
/// - `action`: 管理员交互动作枚举
///
/// # 返回值
/// 若属于已知模块则返回 `Some(AdminCommandKind)`，否则返回 `None`。
pub(super) fn admin_command_kind(action: AdminInputAction) -> Option<AdminCommandKind> {
    // 别名命名与搜索动作归属 targets
    if matches!(
        action,
        AdminInputAction::TargetsAliasName | AdminInputAction::TargetsAliasSearch
    ) {
        return Some(AdminCommandKind::Targets);
    }
    // 命中 targets 规格的动作
    if targets_input_spec_for_admin_action(action).is_some() {
        return Some(AdminCommandKind::Targets);
    }
    // 命中 config 规格的动作
    if config_field_spec_for_admin_action(action).is_some() {
        return Some(AdminCommandKind::Config);
    }
    None
}

/// 将用户在 ForceReply 中输入的文本内容解析为等价的命令行参数列表。
///
/// 例如：
/// - 用户回复 `-100123`，动作是 `TargetsSetDefault` -> 解析为 `["/targets", "set-default", "-100123"]`
/// - 动作是 `ConfigSetJobConcurrency`，输入 `4` -> 解析为 `["/config", "set", "job_concurrency", "4"]`
///
/// # 参数说明
/// - `action`: 当前管理员动作
/// - `input`: 用户回复的原始文本
/// - `_points_target_user_id`: 目标用户 ID（预留扩展参数）
/// - `context_text`: 上下文中已固定的前置参数（例如设置别名第二步中已锁定的 alias 名称）
/// - `_context_i64`: 上下文整型参数（预留）
///
/// # 返回值
/// 若输入格式符合预期，返回包含命令与子参数的 `Some(Vec<String>)`；格式不匹配则返回 `None`。
pub(super) fn parse_admin_input_payload(
    action: AdminInputAction,
    input: &str,
    _points_target_user_id: Option<i64>,
    context_text: Option<&str>,
    _context_i64: Option<i64>,
) -> Option<Vec<String>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    // 按空白字符切分为参数 token 列表
    let parts = trimmed
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    match action {
        // 多步流程中的独立步骤不在此处一次性转换为命令
        AdminInputAction::TargetsAliasName | AdminInputAction::TargetsAliasSearch => None,
        // targets 设置默认目标或设置别名
        AdminInputAction::TargetsSetDefault | AdminInputAction::TargetsSetAlias => {
            let spec = targets_input_spec_for_admin_action(action)?;
            match action {
                // 修改已有 alias 时，alias 已经锁定在草稿上下文中；只接受新的目标值，
                // 防止用户误发两个字段后绕过上下文并意外改名。
                AdminInputAction::TargetsSetAlias if context_text.is_some() => (parts.len() == 1)
                    .then(|| {
                        vec![
                            "/targets".to_owned(),
                            spec.subcommand.to_owned(),
                            context_text.expect("context_text checked above").to_owned(),
                            parts[0].clone(),
                        ]
                    }),
                // 单步直接输入预期参数数量
                _ if parts.len() == spec.expected_parts => {
                    let mut command = vec!["/targets".to_owned(), spec.subcommand.to_owned()];
                    command.extend(parts.iter().cloned());
                    Some(command)
                }
                _ => None,
            }
        }
        // config 系统配置参数设置
        AdminInputAction::ConfigSetJobConcurrency
        | AdminInputAction::ConfigSetFileDeleteDelayMinutes
        | AdminInputAction::ConfigSetFileGcIntervalSeconds
        | AdminInputAction::ConfigSetProgressEditIntervalSeconds
        | AdminInputAction::ConfigSetDownloadsDefaultPageSize
        | AdminInputAction::ConfigSetMenuInputTimeoutSeconds => {
            let spec = config_field_spec_for_admin_action(action)?;
            // 单值参数必须且仅包含一个参数 token
            (parts.len() == 1).then(|| {
                vec![
                    "/config".to_owned(),
                    "set".to_owned(),
                    spec.key.to_owned(),
                    parts[0].clone(),
                ]
            })
        }
    }
}

/// 执行 targets 模块对应的命令逻辑。
///
/// # 参数说明
/// - `app`: 全局应用上下文句柄
/// - `command_owned`: 解析生成的完整参数列表
/// - `request_chat_id`: 触发请求的聊天会话 ID
/// - `client_id`: TDLib 客户端实例标识符
pub(super) async fn run_existing_targets_command(
    app: &crate::app_context::AppContext,
    command_owned: Vec<String>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    targets::targets_command_on(app, command_refs, request_chat_id, client_id).await
}

/// 执行 config 模块对应的参数配置命令逻辑。
///
/// # 参数说明
/// - `app`: 全局应用上下文句柄
/// - `command_owned`: 解析生成的完整参数列表
/// - `request_chat_id`: 触发请求的聊天会话 ID
/// - `client_id`: TDLib 客户端实例标识符
pub(super) async fn run_existing_config_command(
    app: &crate::app_context::AppContext,
    command_owned: Vec<String>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    config_cmd::config_command_on(app, command_refs, request_chat_id, client_id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 targets 模块管理员输入解析：
    /// 包括单步设置默认目标、单步设置别名、以及第二步携带 context_text 别名时的目标更新解析。
    #[test]
    fn test_parse_admin_input_payload_targets() {
        // 设置默认目标
        assert_eq!(
            parse_admin_input_payload(
                AdminInputAction::TargetsSetDefault,
                "-100123",
                None,
                None,
                None
            ),
            Some(vec![
                "/targets".to_owned(),
                "set-default".to_owned(),
                "-100123".to_owned()
            ])
        );
        // 单步直接输入别名与目标
        assert_eq!(
            parse_admin_input_payload(
                AdminInputAction::TargetsSetAlias,
                "archive -100123",
                None,
                None,
                None
            ),
            Some(vec![
                "/targets".to_owned(),
                "set-alias".to_owned(),
                "archive".to_owned(),
                "-100123".to_owned()
            ])
        );
        // 两步流程中的第二步（已锁定 context_text 为 "archive"）
        assert_eq!(
            parse_admin_input_payload(
                AdminInputAction::TargetsSetAlias,
                "123456",
                None,
                Some("archive"),
                None
            ),
            Some(vec![
                "/targets".to_owned(),
                "set-alias".to_owned(),
                "archive".to_owned(),
                "123456".to_owned(),
            ])
        );
        // 编辑已有 alias 时不接受第二个多余字段，避免覆盖草稿中锁定的 alias
        assert_eq!(
            parse_admin_input_payload(
                AdminInputAction::TargetsSetAlias,
                "other 123456",
                None,
                Some("archive"),
                None
            ),
            None
        );
    }

    /// 测试 config 模块管理员输入解析：
    /// 校验并发任务数及输入超时时间参数的转换。
    #[test]
    fn test_parse_admin_input_payload_config() {
        // 设置并发数
        assert_eq!(
            parse_admin_input_payload(
                AdminInputAction::ConfigSetJobConcurrency,
                "4",
                None,
                None,
                None
            ),
            Some(vec![
                "/config".to_owned(),
                "set".to_owned(),
                "job_concurrency".to_owned(),
                "4".to_owned()
            ])
        );
        // 设置菜单超时时间
        assert_eq!(
            parse_admin_input_payload(
                AdminInputAction::ConfigSetMenuInputTimeoutSeconds,
                "900",
                None,
                None,
                None
            ),
            Some(vec![
                "/config".to_owned(),
                "set".to_owned(),
                "menu_input_timeout_seconds".to_owned(),
                "900".to_owned()
            ])
        );
    }

    /// 测试参数数量不匹配时直接拒绝解析。
    #[test]
    fn test_parse_admin_input_payload_rejects_wrong_arity() {
        assert_eq!(
            parse_admin_input_payload(
                AdminInputAction::TargetsSetAlias,
                "archive",
                None,
                None,
                None
            ),
            None
        );
    }

    /// 测试命令归属大类反查。
    #[test]
    fn test_admin_command_kind_uses_runtime_admin_specs() {
        assert_eq!(
            admin_command_kind(AdminInputAction::TargetsSetDefault),
            Some(AdminCommandKind::Targets)
        );
        assert_eq!(
            admin_command_kind(AdminInputAction::TargetsAliasName),
            Some(AdminCommandKind::Targets)
        );
        assert_eq!(
            admin_command_kind(AdminInputAction::ConfigSetJobConcurrency),
            Some(AdminCommandKind::Config)
        );
    }
}
