// `/config` 命令实现：
// - 仅开放安全可调的运行参数
// - 修改后写入业务数据库并同步刷新内存运行配置

mod callback;

use super::common::{
    CommandStyle, RuntimeAdminHelpCopyButton, RuntimeAdminHelpDescriptor, RuntimeAdminUsageItem,
    build_command_examples, build_runtime_admin_page_intro, config_set_command,
    config_show_command, edit_runtime_admin_interaction_card_or_error, reset_action_title,
    send_runtime_admin_callback_error, updated_action_title,
};
use crate::tgbot::send;
use crate::tgbot::transfer::card;
use callback::{CONFIG_FIELD_SPECS, ConfigCallbackAction, ConfigField, parse_config_callback_data};

pub(in crate::tgbot::transfer::command) use callback::ConfigFieldSpec;
pub(super) use callback::build_config_buttons_on;

/// 根据菜单输入动作反查配置字段规格。
pub(in crate::tgbot::transfer::command) fn config_field_spec_for_admin_action(
    action: crate::tgbot::transfer::command::menu::AdminInputAction,
) -> Option<&'static callback::ConfigFieldSpec> {
    CONFIG_FIELD_SPECS
        .iter()
        .find(|spec| spec.admin_input_action == action)
}

/// `config` 管理页的最小帮助 descriptor。
pub(in crate::tgbot::transfer::command) fn config_help_descriptor() -> RuntimeAdminHelpDescriptor {
    RuntimeAdminHelpDescriptor {
        synopsis: format!(
            "{} [show|reset|set <key> <value>]",
            super::common::command_root("config", CommandStyle::Long)
        ),
        usage_items: vec![
            RuntimeAdminUsageItem {
                command: config_show_command(CommandStyle::Long),
                detail: "显示当前可调配置。".to_owned(),
            },
            RuntimeAdminUsageItem {
                command: format!(
                    "{} reset",
                    super::common::command_root("config", CommandStyle::Long)
                ),
                detail: "把当前运行参数重置为启动配置中的默认值，并立即生效。".to_owned(),
            },
            RuntimeAdminUsageItem {
                command: format!(
                    "{} set <key> <value>",
                    super::common::command_root("config", CommandStyle::Long)
                ),
                detail: "修改并持久化某个可调配置，修改后立即生效。".to_owned(),
            },
        ],
        interaction_items: vec!["按钮进入输入流后，会发送 ForceReply；回复参数即可。".to_owned()],
        example_commands: config_example_commands(),
        help_copy_buttons: {
            let mut buttons = vec![RuntimeAdminHelpCopyButton::new(
                "复制 /config show",
                config_show_command(CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Primary,
            )];
            buttons.extend(CONFIG_FIELD_SPECS.iter().take(1).map(|spec| {
                RuntimeAdminHelpCopyButton::new(
                    "复制并发",
                    config_set_command(spec.key, spec.example_value, CommandStyle::Long),
                    tdlib_rs::enums::ButtonStyle::Default,
                )
            }));
            buttons.push(RuntimeAdminHelpCopyButton::new(
                "复制 /config reset",
                "/config reset",
                tdlib_rs::enums::ButtonStyle::Default,
            ));
            buttons
        },
    }
}

/// `/config` 帮助和页面共用的示例命令。
fn config_example_commands() -> Vec<String> {
    let mut commands = vec![
        config_show_command(CommandStyle::Long),
        "/config reset".to_owned(),
    ];
    commands.extend(
        CONFIG_FIELD_SPECS
            .iter()
            .map(|spec| config_set_command(spec.key, spec.example_value, CommandStyle::Long)),
    );
    commands
}

/// 后台并发允许的最小值。
const JOB_CONCURRENCY_MIN: usize = 1;
/// 后台并发允许的最大值，避免误触把本机和 TDLib 压垮。
const JOB_CONCURRENCY_MAX: usize = 32;
/// 文件引用归零后最小删除延迟分钟数。
const FILE_DELETE_DELAY_MINUTES_MIN: i64 = 0;
/// 文件引用归零后最大删除延迟分钟数。
const FILE_DELETE_DELAY_MINUTES_MAX: i64 = 24 * 60;
/// 文件 GC 最小扫描间隔秒数。
const FILE_GC_INTERVAL_SECONDS_MIN: u64 = 5;
/// 文件 GC 最大扫描间隔秒数。
const FILE_GC_INTERVAL_SECONDS_MAX: u64 = 60 * 60;
/// 进度消息编辑间隔最小秒数。
const PROGRESS_EDIT_INTERVAL_SECONDS_MIN: u64 = 1;
/// 进度消息编辑间隔最大秒数。
const PROGRESS_EDIT_INTERVAL_SECONDS_MAX: u64 = 60;
/// 下载列表默认分页最小值。
const DOWNLOADS_DEFAULT_PAGE_SIZE_MIN: u64 = 1;
/// 下载列表默认分页最大值。
const DOWNLOADS_DEFAULT_PAGE_SIZE_MAX: u64 = 20;
/// 菜单输入超时最小秒数。
const MENU_INPUT_TIMEOUT_SECONDS_MIN: u64 = 30;
/// 菜单输入超时最大秒数。
const MENU_INPUT_TIMEOUT_SECONDS_MAX: u64 = 24 * 60 * 60;
/// 配置页标题。
const CONFIG_PAGE_TITLE: &str = "运行配置";
/// 配置页简要说明。
const CONFIG_PAGE_DETAIL: &str =
    "按钮可直接微调；点“设并发 / 设删除 / 设GC / 设进度 / 设分页 / 设超时”后回复一个值。";

/// 在指定上下文上执行 `/config`。
pub async fn config_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (reply, rows) = match text.get(1).copied() {
        None => (
            format_current_transfer_config_text_on(app, CONFIG_PAGE_TITLE),
            build_config_buttons_on(app),
        ),
        Some("show") => (
            format_current_transfer_config_text_on(app, CONFIG_PAGE_TITLE),
            build_config_buttons_on(app),
        ),
        Some("reset") => (
            reset_transfer_config_to_default_on(app).await?,
            build_config_buttons_on(app),
        ),
        Some("set") => {
            let key = text
                .get(2)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("usage: /config set <key> <value>"))?;
            let value = text
                .get(3)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("usage: /config set <key> <value>"))?;
            (
                update_transfer_config_on(app, key, value).await?,
                build_config_buttons_on(app),
            )
        }
        Some(other) => anyhow::bail!("unknown config subcommand: {}", other),
    };

    let mut panel = send::ReplyPanel::card(reply);
    for row in rows {
        panel = panel.row(row);
    }
    panel.send(request_chat_id, client_id).await
}

/// 判断 callback payload 是否属于 `/config`。
pub(super) fn is_config_callback_data(data: &str) -> bool {
    callback::is_config_callback_data(data)
}

/// 在指定上下文上处理 `/config` callback。
pub async fn config_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some(action) = parse_config_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("配置按钮参数无效"), client_id).await?;
        return Ok(());
    };

    send::answer_callback_query(update.id, Some(action.started_tip()), client_id).await?;

    let action_result = match action {
        ConfigCallbackAction::Refresh => Ok(()),
        ConfigCallbackAction::Reset => reset_transfer_config_to_default_on(app).await.map(|_| ()),
        ConfigCallbackAction::Adjust { field, delta } => {
            adjust_transfer_config_on(app, field, delta).await
        }
        ConfigCallbackAction::Input { field } => {
            return crate::tgbot::transfer::command::menu::start_admin_input_callback(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                field.spec().admin_input_action,
                client_id,
            )
            .await;
        }
    };
    if let Err(err) = action_result {
        send_config_callback_error(update.chat_id, client_id, &err).await?;
        return Err(err);
    }

    let (text, keyboard) = send::ReplyPanel::card(format_current_transfer_config_text_on(
        app,
        CONFIG_PAGE_TITLE,
    ))
    .rows(build_config_buttons_on(app))
    .into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "运行配置",
        "/config show",
    )
    .await?;
    Ok(())
}

/// 在指定上下文上更新 `transfer_config` 中允许动态调整的字段。
async fn update_transfer_config_on(
    app: &crate::app_context::AppContext,
    key: &str,
    value: &str,
) -> anyhow::Result<String> {
    let mut transfer_config = crate::tgbot::transfer::runtime_config_on(app);
    match key {
        "job_concurrency" => {
            let parsed = value.parse::<usize>()?;
            if !(JOB_CONCURRENCY_MIN..=JOB_CONCURRENCY_MAX).contains(&parsed) {
                anyhow::bail!(
                    "job_concurrency must be between {} and {}",
                    JOB_CONCURRENCY_MIN,
                    JOB_CONCURRENCY_MAX
                );
            }
            transfer_config.job_concurrency = parsed;
        }
        "file_delete_delay_minutes" | "file_delete_delay_hours" => {
            let parsed = value.parse::<i64>()?;
            if !(FILE_DELETE_DELAY_MINUTES_MIN..=FILE_DELETE_DELAY_MINUTES_MAX).contains(&parsed) {
                anyhow::bail!(
                    "file_delete_delay_minutes must be between {} and {}",
                    FILE_DELETE_DELAY_MINUTES_MIN,
                    FILE_DELETE_DELAY_MINUTES_MAX
                );
            }
            transfer_config.file_delete_delay_minutes = parsed;
        }
        "file_gc_interval_seconds" => {
            let parsed = value.parse::<u64>()?;
            if !(FILE_GC_INTERVAL_SECONDS_MIN..=FILE_GC_INTERVAL_SECONDS_MAX).contains(&parsed) {
                anyhow::bail!(
                    "file_gc_interval_seconds must be between {} and {}",
                    FILE_GC_INTERVAL_SECONDS_MIN,
                    FILE_GC_INTERVAL_SECONDS_MAX
                );
            }
            transfer_config.file_gc_interval_seconds = parsed;
        }
        "progress_edit_interval_seconds" => {
            let parsed = value.parse::<u64>()?;
            if !(PROGRESS_EDIT_INTERVAL_SECONDS_MIN..=PROGRESS_EDIT_INTERVAL_SECONDS_MAX)
                .contains(&parsed)
            {
                anyhow::bail!(
                    "progress_edit_interval_seconds must be between {} and {}",
                    PROGRESS_EDIT_INTERVAL_SECONDS_MIN,
                    PROGRESS_EDIT_INTERVAL_SECONDS_MAX
                );
            }
            transfer_config.progress_edit_interval_seconds = parsed;
        }
        "downloads_default_page_size" => {
            let parsed = value.parse::<u64>()?;
            if !(DOWNLOADS_DEFAULT_PAGE_SIZE_MIN..=DOWNLOADS_DEFAULT_PAGE_SIZE_MAX)
                .contains(&parsed)
            {
                anyhow::bail!(
                    "downloads_default_page_size must be between {} and {}",
                    DOWNLOADS_DEFAULT_PAGE_SIZE_MIN,
                    DOWNLOADS_DEFAULT_PAGE_SIZE_MAX
                );
            }
            transfer_config.downloads_default_page_size = parsed;
        }
        "menu_input_timeout_seconds" => {
            let parsed = value.parse::<u64>()?;
            if !(MENU_INPUT_TIMEOUT_SECONDS_MIN..=MENU_INPUT_TIMEOUT_SECONDS_MAX).contains(&parsed)
            {
                anyhow::bail!(
                    "menu_input_timeout_seconds must be between {} and {}",
                    MENU_INPUT_TIMEOUT_SECONDS_MIN,
                    MENU_INPUT_TIMEOUT_SECONDS_MAX
                );
            }
            transfer_config.menu_input_timeout_seconds = parsed;
        }
        _ => anyhow::bail!("unsupported config key: {}", key),
    }

    crate::tgbot::transfer::save_transfer_runtime_config(&transfer_config).await?;
    crate::tgbot::transfer::update_runtime_config_on(app, transfer_config.clone());
    // 这里只允许修改非敏感运行参数，因此 key/value 可以安全记录，便于追踪运行时变更。
    tracing::info!(key, value, "transfer runtime config updated");

    Ok(format_transfer_config_text(
        &format!("{}：{} = {}", updated_action_title("运行配置"), key, value),
        &transfer_config,
    ))
}

/// 在指定上下文上按按钮小步调整运行配置。
async fn adjust_transfer_config_on(
    app: &crate::app_context::AppContext,
    field: ConfigField,
    delta: i64,
) -> anyhow::Result<()> {
    let mut transfer_config = crate::tgbot::transfer::runtime_config_on(app);
    match field {
        ConfigField::JobConcurrency => {
            let current = i64::try_from(transfer_config.job_concurrency)?;
            transfer_config.job_concurrency = clamp_i64(
                current + delta,
                JOB_CONCURRENCY_MIN as i64,
                JOB_CONCURRENCY_MAX as i64,
            ) as usize;
        }
        ConfigField::FileDeleteDelayMinutes => {
            let current = transfer_config.file_delete_delay_minutes;
            transfer_config.file_delete_delay_minutes = clamp_i64(
                current + delta,
                FILE_DELETE_DELAY_MINUTES_MIN,
                FILE_DELETE_DELAY_MINUTES_MAX,
            );
        }
        ConfigField::FileGcIntervalSeconds => {
            let current = i64::try_from(transfer_config.file_gc_interval_seconds)?;
            transfer_config.file_gc_interval_seconds = clamp_i64(
                current + delta,
                FILE_GC_INTERVAL_SECONDS_MIN as i64,
                FILE_GC_INTERVAL_SECONDS_MAX as i64,
            ) as u64;
        }
        ConfigField::ProgressEditIntervalSeconds => {
            let current = i64::try_from(transfer_config.progress_edit_interval_seconds)?;
            transfer_config.progress_edit_interval_seconds = clamp_i64(
                current + delta,
                PROGRESS_EDIT_INTERVAL_SECONDS_MIN as i64,
                PROGRESS_EDIT_INTERVAL_SECONDS_MAX as i64,
            ) as u64;
        }
        ConfigField::DownloadsDefaultPageSize => {
            let current = i64::try_from(transfer_config.downloads_default_page_size)?;
            transfer_config.downloads_default_page_size = clamp_i64(
                current + delta,
                DOWNLOADS_DEFAULT_PAGE_SIZE_MIN as i64,
                DOWNLOADS_DEFAULT_PAGE_SIZE_MAX as i64,
            ) as u64;
        }
        ConfigField::MenuInputTimeoutSeconds => {
            let current = i64::try_from(transfer_config.menu_input_timeout_seconds)?;
            transfer_config.menu_input_timeout_seconds = clamp_i64(
                current + delta,
                MENU_INPUT_TIMEOUT_SECONDS_MIN as i64,
                MENU_INPUT_TIMEOUT_SECONDS_MAX as i64,
            ) as u64;
        }
    }

    crate::tgbot::transfer::save_transfer_runtime_config(&transfer_config).await?;
    crate::tgbot::transfer::update_runtime_config_on(app, transfer_config);
    tracing::info!(
        field = field.key(),
        delta,
        "transfer runtime config adjusted by callback"
    );
    Ok(())
}

/// 在指定上下文上把运行配置重置为启动配置里的默认值。
async fn reset_transfer_config_to_default_on(
    app: &crate::app_context::AppContext,
) -> anyhow::Result<String> {
    let transfer_config = crate::tgbot::transfer::runtime_default_config_on(app);
    crate::tgbot::transfer::save_transfer_runtime_config(&transfer_config).await?;
    crate::tgbot::transfer::update_runtime_config_on(app, transfer_config.clone());
    tracing::info!("transfer runtime config reset to startup defaults");

    Ok(format_transfer_config_text(
        &reset_action_title("运行配置"),
        &transfer_config,
    ))
}

/// 配置按钮失败提示。
///
/// callback 已经先 ACK，失败时不能再 answer 同一个 callback，因此发送一条短卡片说明错误。
async fn send_config_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_runtime_admin_callback_error(request_chat_id, client_id, "运行配置", err).await
}

/// 把运行时配置格式化成当前卡片文本。
///
/// 菜单页在已经拿到 `AppContext` 时优先用这个版本，避免重复抓全局。
pub(super) fn format_current_transfer_config_text_on(
    app: &crate::app_context::AppContext,
    title: &str,
) -> String {
    format_transfer_config_text(title, &crate::tgbot::transfer::runtime_config_on(app))
}

/// 格式化当前可调配置。
fn format_transfer_config_text(title: &str, config: &crate::config::TransferConfig) -> String {
    let mut lines = build_runtime_admin_page_intro(title, CONFIG_PAGE_DETAIL);
    lines.extend([
        card::section("运行参数"),
        card::field("job_concurrency", config.job_concurrency),
        card::field(
            "file_delete_delay_minutes",
            config.file_delete_delay_minutes,
        ),
        card::field("file_gc_interval_seconds", config.file_gc_interval_seconds),
        card::field(
            "progress_edit_interval_seconds",
            config.progress_edit_interval_seconds,
        ),
        card::field(
            "downloads_default_page_size",
            config.downloads_default_page_size,
        ),
        card::field(
            "menu_input_timeout_seconds",
            config.menu_input_timeout_seconds,
        ),
    ]);
    lines.extend(build_command_examples(config_example_commands()));
    lines.join("\n")
}

/// 把整数限制在安全区间内。
fn clamp_i64(value: i64, min: i64, max: i64) -> i64 {
    value.clamp(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 配置页文本应包含主要字段与命令示例。
    #[test]
    fn test_format_config_text_contains_sections() {
        let cfg = crate::config::TransferConfig {
            job_concurrency: 2,
            file_delete_delay_minutes: 2,
            file_gc_interval_seconds: 60,
            ..Default::default()
        };
        let text = format_transfer_config_text("当前可调配置", &cfg);
        assert!(text.contains("job_concurrency：‹2›"));
        assert!(text.contains("file_delete_delay_minutes：‹2›"));
        assert!(text.contains("file_gc_interval_seconds：‹60›"));
        assert!(text.contains("progress_edit_interval_seconds"));
        assert!(text.contains("downloads_default_page_size"));
        assert!(text.contains("menu_input_timeout_seconds"));
        assert!(text.contains("/config reset"));
        assert!(text.contains("/config show"));
    }

    // 按钮调整必须做边界限制，避免误触后出现 0 并发或过短 GC。
    #[test]
    fn test_clamp_i64() {
        assert_eq!(clamp_i64(0, 1, 32), 1);
        assert_eq!(clamp_i64(33, 1, 32), 32);
        assert_eq!(clamp_i64(10, 1, 32), 10);
    }
}
