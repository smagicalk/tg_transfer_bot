// `/config` 命令实现：
// - 仅开放安全可调的运行参数
// - 修改后写入业务数据库并同步刷新内存运行配置

mod callback;

use super::common::{
    CommandStyle, RuntimeAdminHelpDescriptor, RuntimeAdminUsageItem,
    build_runtime_admin_back_menu_row, build_runtime_admin_detail_text,
    build_runtime_admin_page_intro, build_runtime_admin_section_block, config_set_command,
    config_show_command, edit_runtime_admin_interaction_card_or_error, reset_action_title,
    send_runtime_admin_callback_error, updated_action_title,
};
use super::menu::build_menu_config_callback_data;
use crate::tgbot::send;
use crate::tgbot::transfer::card;
use callback::{CONFIG_FIELD_SPECS, ConfigCallbackAction, parse_config_callback_data};

pub(super) use callback::build_config_buttons_on;
pub(in crate::tgbot::transfer::command) use callback::{ConfigField, ConfigFieldSpec};

/// 根据菜单输入动作反查配置字段规格。
pub(in crate::tgbot::transfer::command) fn config_field_spec_for_admin_action(
    action: crate::tgbot::transfer::command::menu::AdminInputAction,
) -> Option<&'static callback::ConfigFieldSpec> {
    CONFIG_FIELD_SPECS
        .iter()
        .find(|spec| spec.admin_input_action == action)
}

/// 返回 `/config set` 当前支持的全部动态字段规格。
///
/// 菜单页、帮助页和输入流都应尽量从同一份字段定义读取，避免字段名漂移。
pub(in crate::tgbot::transfer::command) fn config_field_specs()
-> &'static [callback::ConfigFieldSpec] {
    CONFIG_FIELD_SPECS
}

/// 构造配置页共用的“可调项”摘要区块。
///
/// 菜单页和 help 详情页都应直接复用这份字段摘要，避免字段增减时两边漂移。
pub(in crate::tgbot::transfer::command) fn config_summary_lines() -> Vec<String> {
    build_runtime_admin_section_block(
        "可调字段",
        config_field_specs()
            .iter()
            .map(|spec| card::field(spec.short_label, spec.key)),
    )
}

/// `config` 页在菜单和帮助详情里共用的开场说明。
pub(in crate::tgbot::transfer::command) fn config_intro_lines() -> Vec<String> {
    vec![
        "先看字段当前值，再进入字段详情页回复新值。".to_owned(),
        "字段详情页优先引导输入式修改，不再强调碎片化 +/- 微调。".to_owned(),
    ]
}

/// 构造 `/help config` 的入口按钮行。
///
/// 帮助页直接复用配置模块自己的字段规格，避免新增配置字段后只改正文忘改按钮。
pub(in crate::tgbot::transfer::command) fn build_config_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![vec![send::build_callback_button(
        "打开配置页",
        &build_menu_config_callback_data(),
        tdlib_rs::enums::ButtonStyle::Primary,
    )]];

    let buttons = config_field_specs()
        .iter()
        .map(|spec| {
            send::build_callback_button(
                &format!("{}详情", spec.short_label),
                &build_config_field_detail_button_data(spec.field),
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect::<Vec<_>>();

    if buttons.len() >= 2 {
        rows[0].extend(buttons[..2].iter().cloned());
        for chunk in buttons[2..].chunks(3) {
            rows.push(chunk.to_vec());
        }
    } else {
        rows[0].extend(buttons);
    }

    rows
}

/// 构造配置字段详情页按钮数据，供帮助页等外层入口复用。
pub(in crate::tgbot::transfer::command) fn build_config_field_detail_button_data(
    field: ConfigField,
) -> String {
    callback::build_config_detail_callback_data(ConfigCallbackAction::View { field })
}

/// `config` 管理页的最小帮助 descriptor。
pub(in crate::tgbot::transfer::command) fn config_help_descriptor() -> RuntimeAdminHelpDescriptor {
    RuntimeAdminHelpDescriptor {
        purpose: "查看或修改可动态生效的运行配置。",
        summary: "查看或修改可动态生效的运行配置；支持字段详情和输入式设置。",
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
    "先查看字段当前值；常用调整直接点步进按钮，只有精确设置才需要回复新值。";

/// 字段允许范围和单次按钮步长。
fn config_field_adjustment(field: ConfigField) -> (i64, i64, i64) {
    match field {
        ConfigField::JobConcurrency => (JOB_CONCURRENCY_MIN as i64, JOB_CONCURRENCY_MAX as i64, 1),
        ConfigField::FileDeleteDelayMinutes => (
            FILE_DELETE_DELAY_MINUTES_MIN,
            FILE_DELETE_DELAY_MINUTES_MAX,
            1,
        ),
        ConfigField::FileGcIntervalSeconds => (
            FILE_GC_INTERVAL_SECONDS_MIN as i64,
            FILE_GC_INTERVAL_SECONDS_MAX as i64,
            5,
        ),
        ConfigField::ProgressEditIntervalSeconds => (
            PROGRESS_EDIT_INTERVAL_SECONDS_MIN as i64,
            PROGRESS_EDIT_INTERVAL_SECONDS_MAX as i64,
            1,
        ),
        ConfigField::DownloadsDefaultPageSize => (
            DOWNLOADS_DEFAULT_PAGE_SIZE_MIN as i64,
            DOWNLOADS_DEFAULT_PAGE_SIZE_MAX as i64,
            1,
        ),
        ConfigField::MenuInputTimeoutSeconds => (
            MENU_INPUT_TIMEOUT_SECONDS_MIN as i64,
            MENU_INPUT_TIMEOUT_SECONDS_MAX as i64,
            60,
        ),
    }
}

/// 根据字段步长计算下一个值，并钳制在允许范围内。
fn adjusted_config_field_value(current: i64, field: ConfigField, direction: i8) -> i64 {
    let (min, max, step) = config_field_adjustment(field);
    (current + step * i64::from(direction)).clamp(min, max)
}

/// 配置字段详情页。
fn format_config_field_detail_text(
    config: &crate::config::TransferConfig,
    field: ConfigField,
) -> String {
    let spec = field.spec();
    build_runtime_admin_detail_text(
        spec.input_title,
        vec![card::field(
            spec.key,
            current_config_field_value(config, field),
        )],
        "说明",
        vec![
            "常用调整可直接点击当前值两侧的步进按钮。".to_owned(),
            format!("需要精确数值时，点“{}”后回复新值。", spec.input_label),
            "点“恢复默认值”只恢复当前字段，不会重置其他运行参数。".to_owned(),
        ],
    )
}

/// 配置字段详情页按钮。
fn build_config_field_detail_buttons(
    config: &crate::config::TransferConfig,
    field: ConfigField,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let spec = field.spec();
    let current = current_config_field_value(config, field);
    let (min, max, step) = config_field_adjustment(field);
    let mut adjust_row = Vec::new();
    if current > min {
        adjust_row.push(send::build_callback_button(
            &format!("-{step}"),
            &callback::build_config_detail_callback_data(ConfigCallbackAction::Adjust {
                field,
                direction: -1,
            }),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    adjust_row.push(send::build_callback_button(
        &format!("当前 {current}"),
        &callback::build_config_detail_callback_data(ConfigCallbackAction::View { field }),
        tdlib_rs::enums::ButtonStyle::Primary,
    ));
    if current < max {
        adjust_row.push(send::build_callback_button(
            &format!("+{step}"),
            &callback::build_config_detail_callback_data(ConfigCallbackAction::Adjust {
                field,
                direction: 1,
            }),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    vec![
        adjust_row,
        vec![
            send::build_callback_button(
                spec.input_label,
                &callback::build_config_detail_callback_data(ConfigCallbackAction::Input { field }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "恢复默认值",
                &callback::build_config_detail_callback_data(ConfigCallbackAction::ResetField {
                    field,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_runtime_admin_back_menu_row(send::build_callback_button(
            "返回配置",
            &callback::build_config_detail_callback_data(ConfigCallbackAction::Refresh),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ]
}

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
        Some(other) => anyhow::bail!("unknown config subcommand: {other}"),
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
        ConfigCallbackAction::ConfirmReset => {
            return render_config_reset_confirm_on(update.chat_id, update.message_id, client_id)
                .await;
        }
        ConfigCallbackAction::ResetField { field } => {
            reset_transfer_config_field_to_default_on(app, field)
                .await
                .map(|_| ())
        }
        ConfigCallbackAction::View { field } => {
            return render_config_field_detail_on(
                app,
                update.chat_id,
                update.message_id,
                field,
                client_id,
            )
            .await;
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
        ConfigCallbackAction::Adjust { field, direction } => {
            if let Err(err) = adjust_transfer_config_field_on(app, field, direction).await {
                send_config_callback_error(update.chat_id, client_id, &err).await?;
                return Err(err);
            }
            return render_config_field_detail_on(
                app,
                update.chat_id,
                update.message_id,
                field,
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
    )
    .await?;
    Ok(())
}

/// 原地打开运行配置全量重置确认页。
async fn render_config_reset_confirm_on(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let text = build_runtime_admin_detail_text(
        "确认重置运行配置",
        vec![card::field("范围", "全部动态运行参数")],
        "影响",
        vec![
            "并发、文件清理、进度刷新、分页和菜单超时都会恢复为启动配置默认值。".to_owned(),
            "如果只想恢复一个字段，请返回字段详情使用“恢复默认值”。".to_owned(),
        ],
    );
    let rows = build_config_reset_confirm_buttons();
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "运行配置",
    )
    .await
}

/// 运行配置重置确认页按钮。
fn build_config_reset_confirm_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![send::build_callback_button(
            "确认重置全部",
            &callback::build_config_detail_callback_data(ConfigCallbackAction::Reset),
            tdlib_rs::enums::ButtonStyle::Danger,
        )],
        build_runtime_admin_back_menu_row(send::build_callback_button(
            "取消",
            &callback::build_config_detail_callback_data(ConfigCallbackAction::Refresh),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ]
}

/// 调整单个字段，并复用现有 set 路径完成范围校验、持久化和运行态刷新。
async fn adjust_transfer_config_field_on(
    app: &crate::app_context::AppContext,
    field: ConfigField,
    direction: i8,
) -> anyhow::Result<()> {
    let config = crate::tgbot::transfer::runtime_config_on(app);
    let current = current_config_field_value(&config, field);
    let value = adjusted_config_field_value(current, field, direction);
    update_transfer_config_on(app, field.spec().key, &value.to_string()).await?;
    Ok(())
}

/// 原地刷新配置字段详情，使连续步进不需要返回列表重新进入。
async fn render_config_field_detail_on(
    app: &crate::app_context::AppContext,
    chat_id: i64,
    message_id: i64,
    field: ConfigField,
    client_id: i32,
) -> anyhow::Result<()> {
    let config = crate::tgbot::transfer::runtime_config_on(app);
    let (text, keyboard) = send::ReplyPanel::card(format_config_field_detail_text(&config, field))
        .rows(build_config_field_detail_buttons(&config, field))
        .into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "运行配置",
    )
    .await
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
                    "job_concurrency must be between {JOB_CONCURRENCY_MIN} and {JOB_CONCURRENCY_MAX}"
                );
            }
            transfer_config.job_concurrency = parsed;
        }
        "file_delete_delay_minutes" | "file_delete_delay_hours" => {
            let parsed = value.parse::<i64>()?;
            if !(FILE_DELETE_DELAY_MINUTES_MIN..=FILE_DELETE_DELAY_MINUTES_MAX).contains(&parsed) {
                anyhow::bail!(
                    "file_delete_delay_minutes must be between {FILE_DELETE_DELAY_MINUTES_MIN} and {FILE_DELETE_DELAY_MINUTES_MAX}"
                );
            }
            transfer_config.file_delete_delay_minutes = parsed;
        }
        "file_gc_interval_seconds" => {
            let parsed = value.parse::<u64>()?;
            if !(FILE_GC_INTERVAL_SECONDS_MIN..=FILE_GC_INTERVAL_SECONDS_MAX).contains(&parsed) {
                anyhow::bail!(
                    "file_gc_interval_seconds must be between {FILE_GC_INTERVAL_SECONDS_MIN} and {FILE_GC_INTERVAL_SECONDS_MAX}"
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
                    "progress_edit_interval_seconds must be between {PROGRESS_EDIT_INTERVAL_SECONDS_MIN} and {PROGRESS_EDIT_INTERVAL_SECONDS_MAX}"
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
                    "downloads_default_page_size must be between {DOWNLOADS_DEFAULT_PAGE_SIZE_MIN} and {DOWNLOADS_DEFAULT_PAGE_SIZE_MAX}"
                );
            }
            transfer_config.downloads_default_page_size = parsed;
        }
        "menu_input_timeout_seconds" => {
            let parsed = value.parse::<u64>()?;
            if !(MENU_INPUT_TIMEOUT_SECONDS_MIN..=MENU_INPUT_TIMEOUT_SECONDS_MAX).contains(&parsed)
            {
                anyhow::bail!(
                    "menu_input_timeout_seconds must be between {MENU_INPUT_TIMEOUT_SECONDS_MIN} and {MENU_INPUT_TIMEOUT_SECONDS_MAX}"
                );
            }
            transfer_config.menu_input_timeout_seconds = parsed;
        }
        _ => anyhow::bail!("unsupported config key: {key}"),
    }

    crate::tgbot::transfer::save_transfer_runtime_config(&transfer_config).await?;
    crate::tgbot::transfer::update_runtime_config_on(app, transfer_config.clone());
    // 这里只允许修改非敏感运行参数，因此 key/value 可以安全记录，便于追踪运行时变更。
    tracing::info!(key, value, "transfer runtime config updated");

    Ok(format_transfer_config_text(
        &format!("{}：{key} = {value}", updated_action_title("运行配置")),
        &transfer_config,
    ))
}

/// 只把某一个运行字段恢复成启动配置中的默认值。
async fn reset_transfer_config_field_to_default_on(
    app: &crate::app_context::AppContext,
    field: ConfigField,
) -> anyhow::Result<String> {
    let mut transfer_config = crate::tgbot::transfer::runtime_config_on(app);
    let default_config = crate::tgbot::transfer::runtime_default_config_on(app);

    apply_transfer_config_field_from(&mut transfer_config, &default_config, field);

    crate::tgbot::transfer::save_transfer_runtime_config(&transfer_config).await?;
    crate::tgbot::transfer::update_runtime_config_on(app, transfer_config.clone());
    tracing::info!(
        field = field.spec().key,
        "transfer runtime config field reset to startup default"
    );

    Ok(format_transfer_config_text(
        &reset_action_title(field.spec().key),
        &transfer_config,
    ))
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
    lines.extend(build_runtime_admin_section_block(
        "运行参数",
        vec![
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
        ],
    ));
    lines.join("\n")
}

/// 读取配置字段当前值，用于详情页渲染。
fn current_config_field_value(config: &crate::config::TransferConfig, field: ConfigField) -> i64 {
    match field {
        ConfigField::JobConcurrency => config.job_concurrency as i64,
        ConfigField::FileDeleteDelayMinutes => config.file_delete_delay_minutes,
        ConfigField::FileGcIntervalSeconds => config.file_gc_interval_seconds as i64,
        ConfigField::ProgressEditIntervalSeconds => config.progress_edit_interval_seconds as i64,
        ConfigField::DownloadsDefaultPageSize => config.downloads_default_page_size as i64,
        ConfigField::MenuInputTimeoutSeconds => config.menu_input_timeout_seconds as i64,
    }
}

/// 把指定字段从来源配置复制到目标配置。
fn apply_transfer_config_field_from(
    target: &mut crate::config::TransferConfig,
    source: &crate::config::TransferConfig,
    field: ConfigField,
) {
    match field {
        ConfigField::JobConcurrency => target.job_concurrency = source.job_concurrency,
        ConfigField::FileDeleteDelayMinutes => {
            target.file_delete_delay_minutes = source.file_delete_delay_minutes;
        }
        ConfigField::FileGcIntervalSeconds => {
            target.file_gc_interval_seconds = source.file_gc_interval_seconds;
        }
        ConfigField::ProgressEditIntervalSeconds => {
            target.progress_edit_interval_seconds = source.progress_edit_interval_seconds;
        }
        ConfigField::DownloadsDefaultPageSize => {
            target.downloads_default_page_size = source.downloads_default_page_size;
        }
        ConfigField::MenuInputTimeoutSeconds => {
            target.menu_input_timeout_seconds = source.menu_input_timeout_seconds;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};

    // 配置页默认只展示字段；命令说明通过“查看命令”按钮按需打开。
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
        assert!(!text.contains("/config"));
    }

    // 配置首页只负责字段导航；步进调整下沉到字段详情，全量重置必须明确标注范围。
    #[test]
    fn test_config_buttons_use_detail_navigation_only() {
        let rows = build_config_buttons_on(crate::app_context::app_context().as_ref());
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.iter().any(|label| label.starts_with("并发")));
        assert!(labels.iter().any(|label| label.starts_with("删除")));
        assert!(labels.contains(&"刷新"));
        assert!(labels.contains(&"重置全部"));
        assert!(labels.contains(&"查看命令"));
        assert!(!labels.iter().any(|label| label.starts_with("+")));
        assert!(!labels.iter().any(|label| label.starts_with("-")));
    }

    // 字段详情页里的“恢复默认值”必须只作用于当前字段，不得误导成全局 reset。
    #[test]
    fn test_config_field_detail_buttons_use_field_reset_action() {
        let config = crate::config::TransferConfig::default();
        let rows = build_config_field_detail_buttons(&config, ConfigField::JobConcurrency);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"设并发"));
        assert!(labels.contains(&"恢复默认值"));
        assert!(labels.contains(&"-1"));
        assert!(labels.contains(&"+1"));
        assert!(labels.contains(&format!("当前 {}", config.job_concurrency).as_str()));
        assert!(!labels.contains(&"重置默认"));

        let reset_button = rows
            .iter()
            .flatten()
            .find(|button| button.text == "恢复默认值")
            .expect("detail page should have field reset button");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &reset_button.r#type
        else {
            panic!("field reset button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "cfg:xf:jc");
    }

    // 字段达到最小值时不应继续显示无效减小按钮。
    #[test]
    fn test_config_field_detail_buttons_hide_invalid_decrease() {
        let config = crate::config::TransferConfig {
            job_concurrency: JOB_CONCURRENCY_MIN,
            ..Default::default()
        };

        let rows = build_config_field_detail_buttons(&config, ConfigField::JobConcurrency);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(!labels.contains(&"-1"));
        assert!(labels.contains(&"+1"));
    }

    // 不同字段应使用各自步长，并始终钳制在允许范围内。
    #[test]
    fn test_adjusted_config_field_value_uses_step_and_bounds() {
        assert_eq!(
            adjusted_config_field_value(30, ConfigField::FileGcIntervalSeconds, 1),
            35
        );
        assert_eq!(
            adjusted_config_field_value(900, ConfigField::MenuInputTimeoutSeconds, -1),
            840
        );
        assert_eq!(
            adjusted_config_field_value(JOB_CONCURRENCY_MAX as i64, ConfigField::JobConcurrency, 1,),
            JOB_CONCURRENCY_MAX as i64
        );
        assert_eq!(
            adjusted_config_field_value(
                FILE_DELETE_DELAY_MINUTES_MIN,
                ConfigField::FileDeleteDelayMinutes,
                -1,
            ),
            FILE_DELETE_DELAY_MINUTES_MIN
        );
    }

    // 全量重置必须经过确认页，确认按钮才发送旧执行 payload。
    #[test]
    fn test_config_reset_confirm_buttons() {
        let rows = build_config_reset_confirm_buttons();
        assert_eq!(rows[0][0].text, "确认重置全部");
        assert_eq!(rows[1][0].text, "取消");

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("config reset confirm must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "cfg:x");
    }

    // 字段级恢复默认值只能覆盖当前字段，不能把其它运行参数一并抹掉。
    #[test]
    fn test_apply_transfer_config_field_from_only_updates_selected_field() {
        let mut target = crate::config::TransferConfig {
            job_concurrency: 8,
            file_delete_delay_minutes: 12,
            file_gc_interval_seconds: 99,
            progress_edit_interval_seconds: 7,
            downloads_default_page_size: 15,
            menu_input_timeout_seconds: 333,
        };
        let source = crate::config::TransferConfig {
            job_concurrency: 2,
            file_delete_delay_minutes: 3,
            file_gc_interval_seconds: 30,
            progress_edit_interval_seconds: 4,
            downloads_default_page_size: 10,
            menu_input_timeout_seconds: 900,
        };

        apply_transfer_config_field_from(&mut target, &source, ConfigField::FileGcIntervalSeconds);

        assert_eq!(target.job_concurrency, 8);
        assert_eq!(target.file_delete_delay_minutes, 12);
        assert_eq!(target.file_gc_interval_seconds, 30);
        assert_eq!(target.progress_edit_interval_seconds, 7);
        assert_eq!(target.downloads_default_page_size, 15);
        assert_eq!(target.menu_input_timeout_seconds, 333);
    }
}
