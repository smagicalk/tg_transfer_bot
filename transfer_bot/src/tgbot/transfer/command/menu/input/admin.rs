// `/menu` 中的管理配置单步输入逻辑。
// 这里只负责把 ForceReply 文本转换成现有 `/targets`、`/config` 命令调用。

use crate::tgbot::transfer::command::config_cmd::config_field_spec_for_admin_action;
use crate::tgbot::transfer::command::targets::targets_input_spec_for_admin_action;
use crate::tgbot::transfer::command::{config_cmd, targets};

use super::state::AdminInputAction;

/// 管理输入最终要复用的命令模块。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdminCommandKind {
    Targets,
    Config,
}

/// 根据规格反查输入动作所属命令，避免主输入流程维护重复 action 分类。
pub(super) fn admin_command_kind(action: AdminInputAction) -> Option<AdminCommandKind> {
    if matches!(
        action,
        AdminInputAction::TargetsAliasName | AdminInputAction::TargetsAliasSearch
    ) {
        return Some(AdminCommandKind::Targets);
    }
    if targets_input_spec_for_admin_action(action).is_some() {
        return Some(AdminCommandKind::Targets);
    }
    if config_field_spec_for_admin_action(action).is_some() {
        return Some(AdminCommandKind::Config);
    }
    None
}

/// 把用户回复解析成已有命令参数。
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
    let parts = trimmed
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    match action {
        AdminInputAction::TargetsAliasName | AdminInputAction::TargetsAliasSearch => None,
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
                _ if parts.len() == spec.expected_parts => {
                    let mut command = vec!["/targets".to_owned(), spec.subcommand.to_owned()];
                    command.extend(parts.iter().cloned());
                    Some(command)
                }
                _ => None,
            }
        }
        AdminInputAction::ConfigSetJobConcurrency
        | AdminInputAction::ConfigSetFileDeleteDelayMinutes
        | AdminInputAction::ConfigSetFileGcIntervalSeconds
        | AdminInputAction::ConfigSetProgressEditIntervalSeconds
        | AdminInputAction::ConfigSetDownloadsDefaultPageSize
        | AdminInputAction::ConfigSetMenuInputTimeoutSeconds => {
            let spec = config_field_spec_for_admin_action(action)?;
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

/// 调用已有 `/targets` 命令入口。
pub(super) async fn run_existing_targets_command(
    app: &crate::app_context::AppContext,
    command_owned: Vec<String>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    targets::targets_command_on(app, command_refs, request_chat_id, client_id).await
}

/// 调用已有 `/config` 命令入口。
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

    #[test]
    fn test_parse_admin_input_payload_targets() {
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
        // 编辑已有 alias 时不接受第二个字段，避免覆盖草稿中锁定的 alias。
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

    #[test]
    fn test_parse_admin_input_payload_config() {
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
