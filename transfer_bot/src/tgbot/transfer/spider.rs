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
///
/// 1. 调用 TDLib `get_internal_link_type` 识别链接形态，过滤出仅限消息链接；
/// 2. 调用 `get_message_link_info` 解析出消息实体锚点 `anchor`；
/// 3. 若该消息属于多媒体相册（`media_album_id != 0`），则扫描上下文收集整个相册中的所有媒体消息；
/// 4. 组装成 `TransferBundle` 传递给下游下载和转发工作流。
///
/// # 参数
/// - `source_link`: Telegram 消息链接字符串。
/// - `client_id`: 执行解析读取的 TDLib 客户端 ID。
/// - `source_client_role`: 读取源消息的客户端角色（Bot 或 User）。
///
/// # 返回值
/// - `Ok(TransferBundle)`: 解析到的消息集合包。
/// - `Err(anyhow::Error)`: 链接类型不合法、无法访问私有会话或获取失败。
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

/// bot-first 抓取链接策略。
///
/// 链接源优先尝试让 bot 客户端读取；如果 bot 因无权限无法访问私有群/私有源，
/// 则自动回退（fallback）至用户（user）客户端进行读取解析。
///
/// # 参数
/// - `source_link`: 原始消息链接。
/// - `bot_client_id`: Bot 客户端实例 ID。
/// - `user_client_id`: User 客户端实例 ID。
///
/// # 返回值
/// - `Ok(TransferBundle)`: 成功获取到的消息包（携带具体胜出的角色）。
/// - `Err(anyhow::Error)`: Bot 和 User 均抓取失败时的完整聚合错误。
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
            // user fallback 也失败时保留 bot 的前置错误，否则失败卡片只能看到最后一次 user 错误。
            spider_message(source_link, user_client_id, ClientRole::User)
                .await
                .with_context(|| {
                    format!("bot failed to resolve source link before user fallback: {bot_err:#}")
                })
        }
    }
}

/// 根据 bot 当前可见的一条消息抓取源（适用于用户在会话内直接回复某消息触发 `/transfer` 的场景）。
///
/// # 参数
/// - `chat_id`: 消息所在会话 ID。
/// - `message_id`: 被回复的目标消息 ID。
/// - `client_id`: Bot 客户端实例 ID。
pub(super) async fn spider_bot_visible_message(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<TransferBundle> {
    let message = get_message(chat_id, message_id, client_id).await?;
    bundle_from_bot_visible_anchor(message, client_id).await
}

/// 按 bot 可见入口消息收集单条或相册，并构造最终的数据包。
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

/// 相册场景：向前后拉取历史消息，收集具有相同 `media_album_id` 的所有消息。
///
/// Telegram 的媒体相册（如多张图片/视频组合发送）在底层是多条独立的 `Message`，
/// 具有相同的 `media_album_id`。本函数以给定的 `anchor` 消息为基准，
/// 前后滑动拉取聊天记录，直到相册中的所有成员全部收集齐或达到安全上限。
///
/// # 参数
/// - `anchor`: 首个被定位到的相册成员消息。
/// - `client_id`: TDLib 客户端 ID。
///
/// # 返回值
/// - `Ok(Vec<Message>)`: 按 `message_id` 升序排列的完整相册消息列表。
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

/// 读取指定单条消息的完整 TDLib `Message` 对象。
///
/// # 参数
/// - `chat_id`: 会话 ID。
/// - `message_id`: 消息 ID。
/// - `client_id`: TDLib 客户端 ID。
async fn get_message(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    let message = tdlib_rs::functions::get_message(chat_id, message_id, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::Message::Message(message) = message;
    Ok(*message)
}

/// 从入口锚点消息和消息列表组装生成 `TransferBundle`。
///
/// # 参数
/// - `source_client_role`: 读取成功的客户端角色。
/// - `anchor`: 首条源消息。
/// - `messages`: 包含的所有消息集合。
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
