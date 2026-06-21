// `/menu` 中的管理配置单步输入逻辑。
// 这里只负责把 ForceReply 文本转换成现有 `/targets`、`/acl`、`/billing`、`/config` 命令调用。

use crate::tgbot::transfer::command::acl::acl_input_spec_for_admin_action;
use crate::tgbot::transfer::command::billing::{
    billing_announcement_spec_for_admin_action, billing_numeric_spec_for_admin_action,
};
use crate::tgbot::transfer::command::config_cmd::config_field_spec_for_admin_action;
use crate::tgbot::transfer::command::targets::targets_input_spec_for_admin_action;
use crate::tgbot::transfer::command::{acl, billing, config_cmd, points, targets};

use super::state::AdminInputAction;

/// 管理输入最终要复用的命令模块。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdminCommandKind {
    Targets,
    Acl,
    Billing,
    Config,
    Points,
}

/// 根据规格反查输入动作所属命令，避免主输入流程维护重复 action 分类。
pub(super) fn admin_command_kind(action: AdminInputAction) -> Option<AdminCommandKind> {
    if targets_input_spec_for_admin_action(action).is_some() {
        return Some(AdminCommandKind::Targets);
    }
    if acl_input_spec_for_admin_action(action).is_some() {
        return Some(AdminCommandKind::Acl);
    }
    if billing_numeric_spec_for_admin_action(action).is_some()
        || billing_announcement_spec_for_admin_action(action).is_some()
    {
        return Some(AdminCommandKind::Billing);
    }
    if config_field_spec_for_admin_action(action).is_some() {
        return Some(AdminCommandKind::Config);
    }
    if matches!(
        action,
        AdminInputAction::PointsAddUser | AdminInputAction::PointsSubUser
    ) {
        return Some(AdminCommandKind::Points);
    }
    None
}

/// 把用户回复解析成已有命令参数。
pub(super) fn parse_admin_input_payload(
    action: AdminInputAction,
    input: &str,
    points_target_user_id: Option<i64>,
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
        AdminInputAction::TargetsSetDefault
        | AdminInputAction::TargetsPickDefault
        | AdminInputAction::TargetsSetRoute
        | AdminInputAction::TargetsPickRoute
        | AdminInputAction::TargetsDelRoute
        | AdminInputAction::TargetsSetAlias
        | AdminInputAction::TargetsDelAlias => {
            let spec = targets_input_spec_for_admin_action(action)?;
            (parts.len() == spec.expected_parts).then(|| {
                let mut command = vec!["/targets".to_owned(), spec.subcommand.to_owned()];
                command.extend(parts.iter().cloned());
                command
            })
        }
        AdminInputAction::AclAddAdmin
        | AdminInputAction::AclDelAdmin
        | AdminInputAction::AclAddAllowUser
        | AdminInputAction::AclDelAllowUser
        | AdminInputAction::AclAddBan
        | AdminInputAction::AclDelBan
        | AdminInputAction::AclAddAllowTarget
        | AdminInputAction::AclDelAllowTarget
        | AdminInputAction::AclAddAllowRequest
        | AdminInputAction::AclDelAllowRequest => {
            let spec = acl_input_spec_for_admin_action(action)?;
            (parts.len() == 1).then(|| {
                vec![
                    "/acl".to_owned(),
                    spec.subcommand.to_owned(),
                    parts[0].clone(),
                ]
            })
        }
        AdminInputAction::BillingSetBaseCost
        | AdminInputAction::BillingSetItemCost
        | AdminInputAction::BillingSetInitialUserPoints => {
            let spec = billing_numeric_spec_for_admin_action(action)?;
            (parts.len() == 1).then(|| {
                vec![
                    "/billing".to_owned(),
                    "set".to_owned(),
                    spec.key.to_owned(),
                    parts[0].clone(),
                ]
            })
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
        AdminInputAction::BillingSetAnnouncement => {
            let spec = billing_announcement_spec_for_admin_action(action)?;
            Some(vec![
                "/billing".to_owned(),
                "set".to_owned(),
                spec.key.to_owned(),
                trimmed.to_owned(),
            ])
        }
        AdminInputAction::PointsAddUser | AdminInputAction::PointsSubUser => {
            let mut parts = trimmed.split_whitespace();
            let amount = parts.next()?.to_owned();
            let reason = parts.next().unwrap_or("admin_adjust").to_owned();
            if parts.next().is_some() {
                return None;
            }
            let target_user_id = points_target_user_id?;
            let action_name = match action {
                AdminInputAction::PointsAddUser => "add",
                AdminInputAction::PointsSubUser => "sub",
                _ => unreachable!("checked in outer match"),
            };
            Some(vec![
                "/points".to_owned(),
                action_name.to_owned(),
                target_user_id.to_string(),
                amount,
                reason,
            ])
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

/// 调用已有 `/acl` 命令入口。
pub(super) async fn run_existing_acl_command(
    app: &crate::app_context::AppContext,
    command_owned: Vec<String>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    acl::acl_command_on(app, command_refs, request_chat_id, client_id).await
}

/// 调用已有 `/billing` 命令入口。
pub(super) async fn run_existing_billing_command(
    app: &crate::app_context::AppContext,
    command_owned: Vec<String>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    billing::billing_command_on(app, command_refs, request_chat_id, client_id).await
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

pub(super) async fn run_existing_points_command(
    command_owned: Vec<String>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    points::points_command(command_refs, actor, client_id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_admin_input_payload_targets() {
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::TargetsSetDefault, "-100123", None),
            Some(vec![
                "/targets".to_owned(),
                "set-default".to_owned(),
                "-100123".to_owned()
            ])
        );
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::TargetsSetRoute, "1 -100123", None),
            Some(vec![
                "/targets".to_owned(),
                "set-route".to_owned(),
                "1".to_owned(),
                "-100123".to_owned()
            ])
        );
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::TargetsSetAlias, "archive -100123", None),
            Some(vec![
                "/targets".to_owned(),
                "set-alias".to_owned(),
                "archive".to_owned(),
                "-100123".to_owned()
            ])
        );
    }

    #[test]
    fn test_parse_admin_input_payload_acl() {
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::AclAddAdmin, "123456", None),
            Some(vec![
                "/acl".to_owned(),
                "add-admin".to_owned(),
                "123456".to_owned()
            ])
        );
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::AclAddAllowTarget, "-100123", None),
            Some(vec![
                "/acl".to_owned(),
                "add-allow-target".to_owned(),
                "-100123".to_owned()
            ])
        );
    }

    #[test]
    fn test_parse_admin_input_payload_billing() {
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::BillingSetBaseCost, "2", None),
            Some(vec![
                "/billing".to_owned(),
                "set".to_owned(),
                "base_cost_points".to_owned(),
                "2".to_owned()
            ])
        );
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::BillingSetAnnouncement, "hello world", None),
            Some(vec![
                "/billing".to_owned(),
                "set".to_owned(),
                "announcement_text".to_owned(),
                "hello world".to_owned()
            ])
        );
    }

    #[test]
    fn test_parse_admin_input_payload_config() {
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::ConfigSetJobConcurrency, "4", None),
            Some(vec![
                "/config".to_owned(),
                "set".to_owned(),
                "job_concurrency".to_owned(),
                "4".to_owned()
            ])
        );
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::ConfigSetMenuInputTimeoutSeconds, "900", None),
            Some(vec![
                "/config".to_owned(),
                "set".to_owned(),
                "menu_input_timeout_seconds".to_owned(),
                "900".to_owned()
            ])
        );
    }

    #[test]
    fn test_parse_admin_input_payload_points() {
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::PointsAddUser, "10 bonus", Some(42)),
            Some(vec![
                "/points".to_owned(),
                "add".to_owned(),
                "42".to_owned(),
                "10".to_owned(),
                "bonus".to_owned(),
            ])
        );
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::PointsSubUser, "5", Some(42)),
            Some(vec![
                "/points".to_owned(),
                "sub".to_owned(),
                "42".to_owned(),
                "5".to_owned(),
                "admin_adjust".to_owned(),
            ])
        );
    }

    #[test]
    fn test_parse_admin_input_payload_rejects_wrong_arity() {
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::TargetsSetRoute, "1", None),
            None
        );
        assert_eq!(
            parse_admin_input_payload(AdminInputAction::AclAddAdmin, "1 2", None),
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
            admin_command_kind(AdminInputAction::AclAddAdmin),
            Some(AdminCommandKind::Acl)
        );
        assert_eq!(
            admin_command_kind(AdminInputAction::ConfigSetJobConcurrency),
            Some(AdminCommandKind::Config)
        );
        assert_eq!(
            admin_command_kind(AdminInputAction::BillingSetAnnouncement),
            Some(AdminCommandKind::Billing)
        );
    }
}
