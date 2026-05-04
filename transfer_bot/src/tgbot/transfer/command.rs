// 命令模块入口：
// - 按命令职责拆分子模块
// - 对外保持统一导出，避免上层调用方感知文件结构变化

pub(super) mod common;
mod config_cmd;
mod downloads;
mod help;
mod job;
mod lookup;
mod transfer_cmd;

pub use config_cmd::config_command;
pub use downloads::downloads_command;
pub use help::help_command;
pub use job::job_command;
pub use lookup::lookup_command;
pub use transfer_cmd::transfer_command;

/// callback payload 路由。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallbackRoute {
    Help,
    Downloads,
    Job,
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
        tdlib_rs::enums::CallbackQueryPayload::Data(_) => CallbackRoute::Unknown,
        _ => CallbackRoute::Unsupported,
    }
}

#[cfg(test)]
mod tests {
    use super::{CallbackRoute, classify_callback_route};

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

    fn payload(data: &str) -> tdlib_rs::enums::CallbackQueryPayload {
        tdlib_rs::enums::CallbackQueryPayload::Data(tdlib_rs::types::CallbackQueryPayloadData {
            data: data.to_owned(),
        })
    }
}
