// 源消息抓取模块：
// - 解析 message link
// - 抓取单条消息或相册消息集合

use std::cmp::{max, min};

use anyhow::Context;
use tdlib_rs::enums::{MessageLinkInfo, Messages};

use super::types::TransferBundle;
use crate::tgbot::TdError;

/// 根据 source link 抓取单条消息或整组相册消息。
pub(super) async fn spider_message(
    source_link: String,
    client_id: i32,
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

    let mut messages = vec![anchor.clone()];

    // 相册场景：向前后拉取历史，收集同 media_album_id 消息。
    if anchor.media_album_id != 0 {
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
    }

    Ok(TransferBundle {
        source_chat_id: anchor.chat_id,
        source_message_id: anchor.id,
        source_album_id: anchor.media_album_id,
        messages,
    })
}
