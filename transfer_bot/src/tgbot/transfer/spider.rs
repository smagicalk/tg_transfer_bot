// 源消息抓取模块：
// - 解析 message link
// - 抓取单条消息或相册消息集合

use std::cmp::{max, min};

use anyhow::Context;
use tdlib_rs::enums::{MessageLinkInfo, Messages};

use super::types::TransferBundle;
use crate::config::ClientRole;
use crate::tgbot::TdError;

/// 根据 source link 抓取单条消息或整组相册消息。
pub(super) async fn spider_message(
    source_link: String,
    client_id: i32,
    source_client_role: ClientRole,
) -> anyhow::Result<TransferBundle> {
    let link_type = tdlib_rs::functions::get_internal_link_type(source_link, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;

    // 当前仅支持 message 链接。
    let msg_link = match link_type {
        tdlib_rs::enums::InternalLinkType::Message(m) => m,
        _ => anyhow::bail!("unsupported link type"),
    };

    let link_info = tdlib_rs::functions::get_message_link_info(msg_link.url, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;

    let anchor = match link_info {
        MessageLinkInfo::MessageLinkInfo(info) => info
            .message
            .context("message link info doesn't contain message")?,
    };

    let messages = collect_album_messages(anchor.clone(), client_id).await?;

    // 源链接本身可能指向私有聊天，日志只记录解析后的 chat/message/album 定位。
    tracing::info!(
        source_chat_id = anchor.chat_id,
        source_message_id = anchor.id,
        source_album_id = anchor.media_album_id,
        message_count = messages.len(),
        "source message bundle resolved"
    );

    Ok(bundle_from_messages(source_client_role, anchor, messages))
}

/// bot-first 抓取链接。
///
/// 链接源优先让 bot 读取；bot 无法访问私有源时，再交给 user 读取。
/// 下载/准备阶段仍可能因为 bot 文件权限或状态失败，runner 会再切 user 重新 spider 并迁移缓存 owner。
pub(super) async fn spider_link_bot_first(
    source_link: String,
    bot_client_id: i32,
    user_client_id: i32,
) -> anyhow::Result<TransferBundle> {
    match spider_message(source_link.clone(), bot_client_id, ClientRole::Bot).await {
        Ok(bundle) => Ok(bundle),
        Err(bot_err) => {
            tracing::warn!(
                error = %bot_err,
                "bot failed to resolve source link, fallback to user"
            );
            spider_message(source_link, user_client_id, ClientRole::User).await
        }
    }
}

/// 根据 bot 当前可见的一条消息抓取源。
pub(super) async fn spider_bot_visible_message(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<TransferBundle> {
    let message = get_message(chat_id, message_id, client_id).await?;
    bundle_from_bot_visible_anchor(message, client_id).await
}

/// 按 bot 可见入口消息收集单条或相册。
async fn bundle_from_bot_visible_anchor(
    message: tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<TransferBundle> {
    let messages = collect_album_messages(message.clone(), client_id).await?;
    tracing::info!(
        source_chat_id = message.chat_id,
        source_message_id = message.id,
        source_album_id = message.media_album_id,
        message_count = messages.len(),
        "bot visible source message resolved"
    );
    Ok(bundle_from_messages(ClientRole::Bot, message, messages))
}

/// 相册场景：向前后拉取历史，收集同 media_album_id 消息。
async fn collect_album_messages(
    anchor: tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<Vec<tdlib_rs::types::Message>> {
    let mut messages = vec![anchor.clone()];
    if anchor.media_album_id == 0 {
        return Ok(messages);
    }

    let mut last_count = 0usize;
    let mut same_count = 3;
    loop {
        let history = tdlib_rs::functions::get_chat_history(
            anchor.chat_id,
            anchor.id,
            -20,
            40,
            false,
            client_id,
        )
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;

        let mut min_id = anchor.id;
        let mut max_id = anchor.id;
        let Messages::Messages(list) = history;

        for m in list.messages.into_iter().flatten() {
            if m.media_album_id == anchor.media_album_id {
                if !messages.contains(&m) {
                    messages.push(m);
                }
            } else {
                min_id = min(min_id, m.id);
                max_id = max(max_id, m.id);
            }
        }

        // 连续多轮没有增长就结束扫描。
        if last_count == messages.len() {
            same_count -= 1;
        } else {
            same_count = 3;
            last_count = messages.len();
        }

        if same_count == 0 || last_count >= 35 || (min_id < anchor.id && max_id > anchor.id) {
            break;
        }
    }

    // 保证顺序稳定。
    messages.sort_by_key(|m| m.id);
    Ok(messages)
}

/// 读取指定消息。
async fn get_message(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let message = tdlib_rs::functions::get_message(chat_id, message_id, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::Message::Message(message) = message;
    Ok(message)
}

/// 从入口消息和消息列表构造 bundle。
fn bundle_from_messages(
    source_client_role: ClientRole,
    anchor: tdlib_rs::types::Message,
    messages: Vec<tdlib_rs::types::Message>,
) -> TransferBundle {
    TransferBundle {
        source_client_role,
        source_chat_id: anchor.chat_id,
        source_message_id: anchor.id,
        source_album_id: anchor.media_album_id,
        messages,
    }
}
