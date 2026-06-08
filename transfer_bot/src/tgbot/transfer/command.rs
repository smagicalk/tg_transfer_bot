// 命令模块入口：
// - 按命令职责拆分子模块
// - 对外保持统一导出，避免上层调用方感知文件结构变化

pub(super) mod common;
mod config_cmd;
mod downloads;
mod help;
mod job;
mod lookup;
mod menu;
mod transfer_cmd;

pub use config_cmd::config_command;
pub use downloads::downloads_command;
pub use help::help_command;
pub use job::job_command;
pub use lookup::lookup_command;
pub use menu::{cancel_menu_input, discard_menu_input, handle_menu_text_input, menu_command};
pub use transfer_cmd::transfer_command;

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

/// callback payload 路由。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallbackRoute {
    Help,
    Downloads,
    Job,
    Config,
    Menu,
    Unknown,
    Unsupported,
}

/// 转存模块 inline keyboard 回调统一入口。
///
/// 回调 payload 按前缀路由，避免上层 `tgbot` 入口知道每个命令的内部格式。
pub async fn transfer_callback_query(
    update: tdlib_rs::enums::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let route = classify_callback_route(&update.payload);

    match route {
        CallbackRoute::Help => help::help_callback_query(update, client_id).await,
        CallbackRoute::Downloads => downloads::downloads_callback_query(update, client_id).await,
        CallbackRoute::Job => job::job_callback_query(update, client_id).await,
        CallbackRoute::Config => config_cmd::config_callback_query(update, client_id).await,
        CallbackRoute::Menu => menu::menu_callback_query(update, client_id).await,
        CallbackRoute::Unknown => {
            crate::tgbot::send::answer_callback_query(update.id, Some("未知按钮参数"), client_id)
                .await
        }
        CallbackRoute::Unsupported => {
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
        CallbackRoute, build_downloads_filter_button_data, build_downloads_short_command,
        build_downloads_status_button_data, build_job_pause_button_data,
        build_job_resume_button_data, build_job_status_button_data, build_job_stop_button_data,
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
        assert!(build_downloads_filter_button_data("unknown", 8).is_none());
        assert_eq!(build_downloads_short_command(Some("run")), "/d run");
    }

    fn payload(data: &str) -> tdlib_rs::enums::CallbackQueryPayload {
        tdlib_rs::enums::CallbackQueryPayload::Data(tdlib_rs::types::CallbackQueryPayloadData {
            data: data.to_owned(),
        })
    }
}
