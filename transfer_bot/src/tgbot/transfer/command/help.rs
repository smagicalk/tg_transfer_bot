// `/help` 命令入口。
// 具体文案、按钮和 topic 归一化分别放在子模块，避免帮助文案撑大入口文件。

use crate::tgbot::send;

mod keyboard;
mod text;
mod topic;

#[cfg(test)]
mod tests;

use keyboard::{build_help_detail_buttons, build_help_index_buttons};
use text::{build_help_detail_text, build_help_index_text};

use crate::tgbot::send::send_interaction_error_card;

/// 判断 callback payload 是否属于 `/help`。
pub(super) fn is_help_callback_data(data: &str) -> bool {
    keyboard::is_help_callback_data(data)
}

/// 给其他命令页生成 help 页面切换按钮数据。
pub(super) fn build_help_callback_data(topic: Option<&str>) -> String {
    keyboard::build_help_callback_data(topic)
}

/// `/help` 命令入口。
/// 默认返回命令目录；带命令名时返回该命令的详细帮助。
pub async fn help_command(
    text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let (help_text, rows) = build_help_page(text.get(1).copied(), actor)?;
    let mut panel = send::ReplyPanel::card(help_text);
    for row in rows {
        panel = panel.row(row);
    }
    panel.send(actor.request_chat_id, client_id).await
}

/// `/help` inline keyboard 回调入口。
///
/// help 页只做“原地切换文案”，不会修改任务状态，所以适合使用 callback。
pub async fn help_callback_query(
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some(topic) = keyboard::parse_help_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("帮助按钮参数无效"), client_id).await?;
        return Ok(());
    };

    if let Some(topic) = topic
        && !help_topic_allowed(topic, actor)
    {
        send::answer_callback_query(update.id, Some("没有权限查看该帮助"), client_id).await?;
        return Ok(());
    }

    send::answer_callback_query(update.id, Some("已切换帮助"), client_id).await?;
    let (text, rows) = match build_help_page(topic, actor) {
        Ok(page) => page,
        Err(err) => {
            send_help_callback_error(update.chat_id, client_id, &err).await?;
            return Err(err);
        }
    };
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "帮助刷新失败",
        "帮助页已生成，但原消息编辑失败；请复制错误或重新发送 /help。",
    )
    .await
}

/// 帮助按钮失败提示。
///
/// callback 已经先 ACK，失败时不能再 answer 同一个 callback，因此发送独立错误卡片。
async fn send_help_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_interaction_error_card(
        request_chat_id,
        client_id,
        "帮助刷新失败",
        "帮助页未更新，请检查日志或复制错误信息。",
        err,
    )
    .await
}

/// 构建 help 目录页或详情页，供文本命令和 callback 共用。
fn build_help_page(
    command_name: Option<&str>,
    actor: crate::config::RequestActor,
) -> anyhow::Result<(String, Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>)> {
    // 先归一化再做权限判断，避免带斜杠写法绕过目录页权限控制。
    let command_name = match command_name {
        Some(command_name) => {
            let normalized = topic::normalize_help_topic(command_name)?;
            if !help_topic_allowed(normalized, actor) {
                anyhow::bail!("没有权限查看该帮助: {}", normalized);
            }
            Some(normalized)
        }
        None => None,
    };

    match command_name {
        None => Ok((
            build_help_index_text(actor.is_admin()),
            build_help_index_buttons(actor.is_admin()),
        )),
        Some(command_name) => Ok((
            build_help_detail_text(command_name, actor.is_admin())?,
            build_help_detail_buttons(command_name, actor.is_admin())?,
        )),
    }
}

/// 判断当前身份是否允许查看指定 help topic。
///
/// 普通用户不能执行 `/health`、`/cache`、`/config`，帮助入口也不应把这些管理命令暴露成按钮。
fn help_topic_allowed(command_name: &str, actor: crate::config::RequestActor) -> bool {
    actor.is_admin()
        || !matches!(
            command_name,
            "health" | "cache" | "config" | "targets" | "acl" | "billing"
        )
}

#[cfg(test)]
mod access_tests {
    use super::*;

    fn actor(role: crate::config::ActorRole) -> crate::config::RequestActor {
        crate::config::RequestActor {
            request_chat_id: 100,
            user_id: 100,
            role,
        }
    }

    // 普通用户 help 只能看到自己可执行的命令；admin 可以查看全部管理命令说明。
    #[test]
    fn test_help_topic_allowed_by_actor_role() {
        let user = actor(crate::config::ActorRole::User);
        let admin = actor(crate::config::ActorRole::Admin);

        assert!(help_topic_allowed("transfer", user));
        assert!(help_topic_allowed("points", user));
        assert!(!help_topic_allowed("config", user));
        assert!(!help_topic_allowed("health", user));
        assert!(!help_topic_allowed("cache", user));
        assert!(help_topic_allowed("config", admin));
        assert!(help_topic_allowed("health", admin));
        assert!(help_topic_allowed("cache", admin));
    }

    // `/help <topic>` 必须先归一化再做权限判断；当前仅接受长命令 topic。
    #[test]
    fn test_build_help_page_rejects_short_aliases_for_user() {
        let user = actor(crate::config::ActorRole::User);
        let admin = actor(crate::config::ActorRole::Admin);

        assert!(build_help_page(Some("cfg"), user).is_err());
        assert!(build_help_page(Some("hl"), user).is_err());
        assert!(build_help_page(Some("fc"), user).is_err());
        assert!(build_help_page(Some("bal"), user).is_err());
        assert!(build_help_page(Some("cfg"), admin).is_err());
        assert!(build_help_page(Some("balance"), user).is_ok());
        assert!(build_help_page(Some("config"), admin).is_ok());
    }
}
