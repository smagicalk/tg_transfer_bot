// 命令模块入口：
// - 按命令职责拆分子模块
// - 对外保持统一导出，避免上层调用方感知文件结构变化

mod cache;
pub(super) mod common;
mod config_cmd;
mod downloads;
mod health;
mod help;
mod job;
mod lookup;
mod menu;
mod points;
mod transfer_cmd;

pub use cache::cache_command;
pub use config_cmd::config_command;
pub use downloads::downloads_command;
pub use health::health_command;
pub use help::help_command;
pub use job::job_command;
pub use lookup::lookup_command;
pub use menu::{
    cancel_menu_input, discard_menu_input, discard_menu_input_for_command,
    handle_menu_shared_chat_input, handle_menu_text_input, menu_command,
};
pub use points::{balance_command, points_command};
pub use transfer_cmd::{transfer_bot_message_auto_command, transfer_command};

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

/// 给卡片按钮生成可复制的短 `/downloads` 命令。
///
/// 调用方只传筛选参数，不直接依赖命令内部的 `CommandStyle`，避免非命令模块知道过多实现细节。
pub(in crate::tgbot::transfer) fn build_downloads_short_command(filter: Option<&str>) -> String {
    common::downloads_command(filter, None, None, common::CommandStyle::Short)
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
    cache::build_cache_summary_callback_data()
}

/// callback payload 路由。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallbackRoute {
    Help,
    Downloads,
    Job,
    Config,
    Health,
    Cache,
    Points,
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
    let route = classify_callback_route(&update.payload);
    tracing::debug!(
        route = ?route,
        chat_id = update.chat_id,
        sender_user_id = update.sender_user_id,
        message_id = update.message_id,
        "transfer callback query routed"
    );

    match route {
        CallbackRoute::Help => help::help_callback_query(update, client_id).await,
        CallbackRoute::Downloads => {
            downloads::downloads_callback_query(update, actor, client_id).await
        }
        CallbackRoute::Job => job::job_callback_query(update, actor, client_id).await,
        CallbackRoute::Config if actor.is_admin() => {
            config_cmd::config_callback_query(update, client_id).await
        }
        CallbackRoute::Config => send_permission_denied_callback(update, client_id).await,
        CallbackRoute::Health if actor.is_admin() => {
            health::health_callback_query(update, client_id).await
        }
        CallbackRoute::Health => send_permission_denied_callback(update, client_id).await,
        CallbackRoute::Cache if actor.is_admin() => {
            cache::cache_callback_query(update, client_id).await
        }
        CallbackRoute::Cache => send_permission_denied_callback(update, client_id).await,
        CallbackRoute::Points => points::points_callback_query(update, actor, client_id).await,
        CallbackRoute::Menu => menu::menu_callback_query(update, config, actor, client_id).await,
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

/// 普通用户点击 admin-only 按钮时的统一提示。
async fn send_permission_denied_callback(
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::answer_callback_query(update.id, Some("没有权限执行此操作"), client_id)
        .await
}

/// 根据 callback payload 前缀分类路由。
fn classify_callback_route(payload: &tdlib_rs::enums::CallbackQueryPayload) -> CallbackRoute {
    match payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data)
            if help::is_help_callback_data(&data.data) =>
        {
            CallbackRoute::Help
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
            if points::is_points_callback_data(&data.data) =>
        {
            CallbackRoute::Points
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
        build_downloads_short_command, build_downloads_status_button_data,
        build_health_button_data, build_job_pause_button_data, build_job_resume_button_data,
        build_job_status_button_data, build_job_stop_button_data, build_menu_home_button_data,
        classify_callback_route,
    };

    // callback 分发只看短前缀，具体参数合法性由各命令模块自行校验。
    #[test]
    fn test_classify_callback_route() {
        assert_eq!(
            classify_callback_route(&payload("h:transfer")),
            CallbackRoute::Help
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
            classify_callback_route(&payload("hl:show")),
            CallbackRoute::Health
        );
        assert_eq!(
            classify_callback_route(&payload("c:v:summary:10:1")),
            CallbackRoute::Cache
        );
        assert_eq!(
            classify_callback_route(&payload("pt:p:b:1:10:1")),
            CallbackRoute::Points
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
        assert_eq!(build_downloads_short_command(Some("run")), "/d run");
    }

    // 当前所有 callback 前缀必须互不覆盖，否则统一路由会把按钮分发到错误模块。
    #[test]
    fn test_callback_prefixes_are_unique_by_route() {
        let samples = [
            ("h:transfer", CallbackRoute::Help),
            ("d:f:run:8:1", CallbackRoute::Downloads),
            ("j:p:42", CallbackRoute::Job),
            ("cfg:a:gc:10", CallbackRoute::Config),
            ("hl:show", CallbackRoute::Health),
            ("c:v:summary:10:1", CallbackRoute::Cache),
            ("pt:p:b:1:10:1", CallbackRoute::Points),
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
