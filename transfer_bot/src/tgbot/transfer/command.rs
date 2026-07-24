// 命令模块入口：
// - 按命令职责拆分子模块
// - 对外保持统一导出，避免上层调用方感知文件结构变化

mod auth;
mod cache;
pub(super) mod common;
mod config_cmd;
mod downloads;
mod health;
mod help;
mod job;
mod lookup;
mod menu;
mod targets;
mod transfer_cmd;

pub(in crate::tgbot) use auth::{
    auth_callback_query_on, auth_command_on, handle_auth_shared_user_input,
    handle_auth_text_input_on,
};
pub(in crate::tgbot) use auth::{cancel_auth_input, discard_auth_input_for_command};
pub(in crate::tgbot) use cache::cache_command_on;
pub(in crate::tgbot) use config_cmd::config_command_on;
pub(in crate::tgbot) use downloads::downloads_command_on;
pub(in crate::tgbot) use health::health_command_on;
pub use help::help_command;
pub use job::job_command;
pub(in crate::tgbot) use job::job_command_on;
pub(in crate::tgbot) use lookup::lookup_command_on;
pub(in crate::tgbot) use menu::menu_command_on;
pub(in crate::tgbot) use menu::start_transfer_target_choice_from_bot_message;
pub(in crate::tgbot) use menu::start_transfer_target_choice_from_link_message;
pub use menu::{cancel_menu_input, discard_menu_input, discard_menu_input_for_command};
pub(in crate::tgbot) use menu::{handle_menu_shared_chat_input, handle_menu_text_input_on};
pub(in crate::tgbot) use targets::targets_command_on;
pub(in crate::tgbot) use transfer_cmd::transfer_command_on;
pub(in crate::tgbot) use transfer_cmd::transferable_message_source_location;

/// 给转存结果/进度卡片生成“任务详情”按钮数据。
///
/// 外部模块只需要知道 job_id，不应依赖 `/job` 的内部参数枚举。
pub(in crate::tgbot::transfer) fn build_job_status_button_data(job_id: i64) -> String {
    job::build_job_status_callback_data(job_id)
}

/// 给进度/状态卡片生成“暂停任务”按钮数据。
pub(in crate::tgbot::transfer) fn build_job_pause_button_data(job_id: i64) -> String {
    job::build_job_pause_callback_data(job_id)
}

/// 给进度/状态卡片生成“恢复任务”按钮数据。
pub(in crate::tgbot::transfer) fn build_job_resume_button_data(job_id: i64) -> String {
    job::build_job_resume_callback_data(job_id)
}

/// 给进度/状态卡片生成“停止任务”按钮数据。
pub(in crate::tgbot::transfer) fn build_job_stop_button_data(job_id: i64) -> String {
    job::build_job_stop_callback_data(job_id)
}

/// 给进度卡片生成任务状态对应的列表入口。
///
/// 返回值依次是 `/downloads` 筛选参数和按钮文案；状态映射仍由 `/job` 模块维护，
/// 这样任务详情和进度卡片不会出现两套状态解释。
pub(in crate::tgbot::transfer) fn build_job_list_button_meta(
    status: &str,
) -> (&'static str, &'static str) {
    job::job_list_button_meta(status)
}

/// 给转存结果/进度卡片生成“按任务状态返回列表”的按钮数据。
///
/// 状态到筛选器的映射由 `/downloads` 模块维护，避免各处重复写一套状态表。
pub(in crate::tgbot::transfer) fn build_downloads_status_button_data(
    status: &str,
    limit: u64,
) -> String {
    downloads::build_downloads_return_list_callback_data(status, limit)
}

/// 给转存结果/错误卡片生成“按筛选值进入列表”的按钮数据。
///
/// `filter_value` 使用 `/downloads` 的英文参数值，例如 `done`、`fail`。
pub(in crate::tgbot::transfer) fn build_downloads_filter_button_data(
    filter_value: &str,
    limit: u64,
) -> Option<String> {
    downloads::build_downloads_filter_value_callback_data(filter_value, limit)
}

/// 给固定筛选值页面生成下载列表按钮数据。
///
/// 这里只给写死的合法筛选值使用；如果这里失败，说明代码和 `/downloads`
/// 的筛选协议已经漂移，应在开发阶段直接暴露出来。
pub(in crate::tgbot::transfer) fn require_downloads_filter_button_data(
    filter_value: &str,
    limit: u64,
) -> String {
    build_downloads_filter_button_data(filter_value, limit)
        .unwrap_or_else(|| panic!("downloads filter must exist: {filter_value}"))
}

/// 给结果/失败卡片生成“重新转存”按钮数据。
///
/// 具体上下文仍由消息发送方挂载到 `lookup_retry` 状态中，这里只暴露稳定 callback data。
pub(in crate::tgbot::transfer) fn build_lookup_retry_transfer_button_data() -> String {
    lookup::build_lookup_retry_transfer_callback_data()
}

/// 给外层错误卡片生成“打开帮助”按钮数据。
pub(in crate::tgbot) fn build_help_button_data(topic: Option<&str>) -> String {
    help::build_help_callback_data(topic)
}

/// 给结果、进度和错误卡片生成“查看命令”按钮数据。
pub(in crate::tgbot) fn build_help_message_button_data(topic: Option<&str>) -> String {
    help::build_help_message_callback_data(topic)
}

/// 构造不会覆盖原卡片的“查看命令”按钮。
pub(in crate::tgbot) fn build_view_commands_button(
    topic: Option<&str>,
) -> tdlib_rs::types::InlineKeyboardButton {
    crate::tgbot::send::build_callback_button(
        "查看命令",
        &build_help_message_button_data(topic),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 给外层错误卡片生成“返回菜单”按钮数据。
pub(in crate::tgbot) fn build_menu_home_button_data_for_outer() -> String {
    menu::build_menu_home_callback_data()
}

/// 给外层错误卡片生成“开始转存”按钮数据。
pub(in crate::tgbot) fn build_menu_new_transfer_button_data_for_outer() -> String {
    menu::build_menu_new_transfer_callback_data()
}

/// 给各类状态卡片生成“返回菜单”按钮数据。
pub(in crate::tgbot::transfer) fn build_menu_home_button_data() -> String {
    menu::build_menu_home_callback_data()
}

/// 给菜单页生成“运行健康”按钮数据。
pub(in crate::tgbot::transfer) fn build_health_button_data() -> String {
    health::build_health_callback_data()
}

/// 给菜单页生成“文件缓存”按钮数据。
pub(in crate::tgbot::transfer) fn build_cache_button_data() -> String {
    cache::build_cache_default_callback_data()
}

/// callback payload 路由。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallbackRoute {
    Retransfer,
    Auth,
    Help,
    Lookup,
    Downloads,
    Job,
    Config,
    Targets,
    Health,
    Cache,
    Menu,
    Unknown,
    Unsupported,
}

/// 转存模块 inline keyboard 回调统一入口。
///
/// 回调 payload 按前缀路由，避免上层 `tgbot` 入口知道每个命令的内部格式。
pub async fn transfer_callback_query(
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: std::sync::Arc<crate::config::BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let app_context = crate::app_context::app_context();
    transfer_callback_query_on(app_context.as_ref(), update, config, actor, client_id).await
}

/// 在指定上下文上处理转存模块 inline keyboard 回调。
pub(in crate::tgbot) async fn transfer_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: std::sync::Arc<crate::config::BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let route = classify_callback_route(&update.payload);
    tracing::debug!(
        route = ?route,
        chat_id = update.chat_id,
        sender_user_id = update.sender_user_id,
        message_id = update.message_id,
        "transfer callback query routed"
    );

    match route {
        CallbackRoute::Retransfer => {
            transfer_cmd::retransfer_callback_query_on(app, update, config, client_id).await
        }
        CallbackRoute::Auth => auth_callback_query_on(app, update, config, actor, client_id).await,
        CallbackRoute::Help => help::help_callback_query(update, actor, client_id).await,
        CallbackRoute::Lookup => {
            lookup::lookup_callback_query_on(app, update, config, actor, client_id).await
        }
        CallbackRoute::Downloads => {
            downloads::downloads_callback_query_on(app, update, actor, client_id).await
        }
        CallbackRoute::Job => job::job_callback_query_on(app, update, actor, client_id).await,
        CallbackRoute::Config => config_cmd::config_callback_query_on(app, update, client_id).await,
        CallbackRoute::Targets => targets::targets_callback_query_on(app, update, client_id).await,
        CallbackRoute::Health => health::health_callback_query_on(app, update, client_id).await,
        CallbackRoute::Cache => cache::cache_callback_query_on(app, update, client_id).await,
        CallbackRoute::Menu => {
            menu::menu_callback_query_on(app, update, config, actor, client_id).await
        }
        CallbackRoute::Unknown => {
            tracing::warn!(
                chat_id = update.chat_id,
                sender_user_id = update.sender_user_id,
                message_id = update.message_id,
                "unknown transfer callback payload"
            );
            crate::tgbot::send::answer_callback_query(update.id, Some("未知按钮参数"), client_id)
                .await
        }
        CallbackRoute::Unsupported => {
            tracing::warn!(
                chat_id = update.chat_id,
                sender_user_id = update.sender_user_id,
                message_id = update.message_id,
                "unsupported transfer callback payload"
            );
            crate::tgbot::send::answer_callback_query(
                update.id,
                Some("暂不支持这种按钮类型"),
                client_id,
            )
            .await
        }
    }
}

/// 根据 callback payload 前缀分类路由。
fn classify_callback_route(payload: &tdlib_rs::enums::CallbackQueryPayload) -> CallbackRoute {
    match payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if transfer_cmd::is_retransfer_callback_data(&data.data) =>
        {
            CallbackRoute::Retransfer
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if auth::is_auth_callback_data(&data.data) =>
        {
            CallbackRoute::Auth
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if help::is_help_callback_data(&data.data) =>
        {
            CallbackRoute::Help
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if lookup::is_lookup_callback_data(&data.data) =>
        {
            CallbackRoute::Lookup
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if downloads::is_downloads_callback_data(&data.data) =>
        {
            CallbackRoute::Downloads
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if job::is_job_callback_payload(&data.data) =>
        {
            CallbackRoute::Job
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if config_cmd::is_config_callback_data(&data.data) =>
        {
            CallbackRoute::Config
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if targets::is_targets_callback_data(&data.data) =>
        {
            CallbackRoute::Targets
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if health::is_health_callback_data(&data.data) =>
        {
            CallbackRoute::Health
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if cache::is_cache_callback_data(&data.data) =>
        {
            CallbackRoute::Cache
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if menu::is_menu_callback_data(&data.data) =>
        {
            CallbackRoute::Menu
        }
        tdlib_rs::enums::CallbackQueryPayload::Data(_) => CallbackRoute::Unknown,
        _ => CallbackRoute::Unsupported,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CallbackRoute, build_cache_button_data, build_downloads_filter_button_data,
        build_downloads_status_button_data, build_health_button_data, build_job_pause_button_data,
        build_job_resume_button_data, build_job_status_button_data, build_job_stop_button_data,
        build_menu_home_button_data, classify_callback_route,
    };

    // callback 分发只看短前缀，具体参数合法性由各命令模块自行校验。
    #[test]
    fn test_classify_callback_route() {
        assert_eq!(
            classify_callback_route(&payload("tr:again")),
            CallbackRoute::Retransfer
        );
        assert_eq!(
            classify_callback_route(&payload("au:refresh")),
            CallbackRoute::Auth
        );
        assert_eq!(
            classify_callback_route(&payload("h:transfer")),
            CallbackRoute::Help
        );
        assert_eq!(
            classify_callback_route(&payload("lk:rt")),
            CallbackRoute::Lookup
        );
        assert_eq!(
            classify_callback_route(&payload("d:r:run:8:1")),
            CallbackRoute::Downloads
        );
        assert_eq!(
            classify_callback_route(&payload("j:st:42")),
            CallbackRoute::Job
        );
        assert_eq!(
            classify_callback_route(&payload("cfg:r")),
            CallbackRoute::Config
        );
        assert_eq!(
            classify_callback_route(&payload("tcfg:r")),
            CallbackRoute::Targets
        );
        assert_eq!(
            classify_callback_route(&payload("hl:show")),
            CallbackRoute::Health
        );
        assert_eq!(
            classify_callback_route(&payload("c:v:summary:10:1")),
            CallbackRoute::Cache
        );
        assert_eq!(
            classify_callback_route(&payload("m:home")),
            CallbackRoute::Menu
        );
        assert_eq!(
            classify_callback_route(&payload("x:bad")),
            CallbackRoute::Unknown
        );
        assert_eq!(
            classify_callback_route(&tdlib_rs::enums::CallbackQueryPayload::Game(
                tdlib_rs::types::CallbackQueryPayloadGame::default(),
            )),
            CallbackRoute::Unsupported
        );
    }

    // 卡片按钮使用的 callback 包装应继续落到统一 callback 路由。
    #[test]
    fn test_card_callback_builders_route_back_to_commands() {
        let job_data = build_job_status_button_data(42);
        let pause_data = build_job_pause_button_data(42);
        let resume_data = build_job_resume_button_data(42);
        let stop_data = build_job_stop_button_data(42);
        let running_data = build_downloads_status_button_data("running", 8);
        let done_data =
            build_downloads_filter_button_data("done", 8).expect("done filter must exist");
        let health_data = build_health_button_data();
        let cache_data = build_cache_button_data();
        let menu_data = build_menu_home_button_data();

        assert_eq!(
            classify_callback_route(&payload(&job_data)),
            CallbackRoute::Job
        );
        assert_eq!(
            classify_callback_route(&payload(&pause_data)),
            CallbackRoute::Job
        );
        assert_eq!(
            classify_callback_route(&payload(&resume_data)),
            CallbackRoute::Job
        );
        assert_eq!(
            classify_callback_route(&payload(&stop_data)),
            CallbackRoute::Job
        );
        assert_eq!(stop_data, "j:sc:42");
        assert_eq!(
            classify_callback_route(&payload(&running_data)),
            CallbackRoute::Downloads
        );
        assert_eq!(
            classify_callback_route(&payload(&done_data)),
            CallbackRoute::Downloads
        );
        assert_eq!(
            classify_callback_route(&payload(&health_data)),
            CallbackRoute::Health
        );
        assert_eq!(
            classify_callback_route(&payload(&cache_data)),
            CallbackRoute::Cache
        );
        assert_eq!(
            classify_callback_route(&payload(&menu_data)),
            CallbackRoute::Menu
        );
        assert!(build_downloads_filter_button_data("unknown", 8).is_none());
    }

    // 当前所有 callback 前缀必须互不覆盖，否则统一路由会把按钮分发到错误模块。
    #[test]
    fn test_callback_prefixes_are_unique_by_route() {
        let samples = [
            ("au:add", CallbackRoute::Auth),
            ("h:transfer", CallbackRoute::Help),
            ("lk:rt", CallbackRoute::Lookup),
            ("d:f:run:8:1", CallbackRoute::Downloads),
            ("j:p:42", CallbackRoute::Job),
            ("j:sc:42", CallbackRoute::Job),
            ("cfg:a:gc:10", CallbackRoute::Config),
            ("tcfg:r", CallbackRoute::Targets),
            ("hl:show", CallbackRoute::Health),
            ("c:v:summary:10:1", CallbackRoute::Cache),
            ("m:t", CallbackRoute::Menu),
        ];

        for (data, expected_route) in samples {
            assert_eq!(classify_callback_route(&payload(data)), expected_route);
        }
    }

    fn payload(data: &str) -> tdlib_rs::enums::CallbackQueryPayload {
        tdlib_rs::enums::CallbackQueryPayload::Data(tdlib_rs::types::CallbackQueryPayloadData {
            data: data.to_owned(),
        })
    }
}
