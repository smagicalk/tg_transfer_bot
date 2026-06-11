// 上传阶段：
// - 单条消息使用 send_message
// - 多条消息使用 send_message_album，超过 Telegram 上限时按 10 条分批发送
// - 返回每个上传分组的 TDLib 消息 ID，结果链接由 result_link 模块单独生成

use crate::tgbot::TdError;

use super::super::file::{PreparedUpload, UploadKind};

/// Telegram media group / album 一次最多包含 10 个媒体项。
const TELEGRAM_ALBUM_MAX_ITEMS: usize = 10;

/// 上传准备好的消息集合：
/// - 1 条 => send_message
/// - 多条 => send_message_album
///
/// 多条转存会尽量保持 album 形态：
/// - 多条内容不能组成合法 album 时直接失败，不降级逐条发送。
/// - 超过 Telegram album 上限时按 10 条一组分批发送，每组仍然是 album。
/// - 分组会避免最后只剩 1 条，例如 11 条会拆成 9 + 2，而不是 10 + 1。
pub(super) async fn upload_prepared(
    target_chat_id: i64,
    prepared: &[(i64, PreparedUpload)],
    client_id: i32,
) -> anyhow::Result<UploadResult> {
    if prepared.is_empty() {
        anyhow::bail!("no prepared item to upload");
    }

    if prepared.len() == 1 {
        tracing::info!(
            target_chat_id,
            client_id,
            "uploading single prepared message"
        );
        let content = prepared[0].1.input_content.clone();
        let sent = tdlib_rs::functions::send_message(
            target_chat_id,
            None,
            None,
            None,
            None,
            content,
            client_id,
        )
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;
        let tdlib_rs::enums::Message::Message(message) = sent;
        let message = crate::tgbot::send::wait_for_sent_message(message, client_id).await?;
        return Ok(UploadResult {
            entries: vec![UploadedResultEntry {
                message_id: message.id,
                is_album: false,
                item_count: 1,
            }],
        });
    }

    let kinds = prepared.iter().map(|(_, p)| p.kind).collect::<Vec<_>>();
    validate_album_kinds(&kinds)?;

    // TDLib 单个 album 最多 10 条；超过时分成多个分组，避免 10 条以上直接失败。
    let contents = prepared
        .iter()
        .map(|(_, p)| p.input_content.clone())
        .collect::<Vec<_>>();
    let chunk_sizes = album_chunk_sizes(contents.len());
    let mut entries = Vec::with_capacity(chunk_sizes.len());
    let mut offset = 0usize;
    for (chunk_index, chunk_size) in chunk_sizes.iter().copied().enumerate() {
        let chunk = &contents[offset..offset + chunk_size];
        offset += chunk_size;

        tracing::info!(
            target_chat_id,
            client_id,
            chunk_index = chunk_index + 1,
            chunk_size,
            total_items = contents.len(),
            "uploading prepared album chunk"
        );
        let rs = tdlib_rs::functions::send_message_album(
            target_chat_id,
            None,
            None,
            None,
            chunk.to_vec(),
            client_id,
        )
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))?;
        let tdlib_rs::enums::Messages::Messages(messages) = rs;
        let msg = messages
            .messages
            .first()
            .and_then(|msg| msg.clone())
            .ok_or_else(|| anyhow::anyhow!("send_message_album returned no message id"))?;
        let msg = crate::tgbot::send::wait_for_sent_message(msg, client_id).await?;
        entries.push(UploadedResultEntry {
            message_id: msg.id,
            is_album: true,
            item_count: chunk.len() as i32,
        });
    }
    if entries.is_empty() {
        anyhow::bail!("upload completed without result message id");
    }
    Ok(UploadResult { entries })
}

/// 上传结果摘要。
#[derive(Debug, Clone)]
pub(super) struct UploadResult {
    /// 上传产生的结果入口。超过 10 条媒体时会有多个入口。
    pub entries: Vec<UploadedResultEntry>,
}

/// 单个上传分组的结果入口。
#[derive(Debug, Clone, Copy)]
pub(super) struct UploadedResultEntry {
    /// 入口消息 ID；album 使用分组首条消息。
    pub message_id: i64,
    /// 该入口是否是 album。
    pub is_album: bool,
    /// 该入口覆盖的源条目数。
    pub item_count: i32,
}

/// 计算 album 上传分组。
///
/// Telegram 单个 album 最多 10 条，且 album 至少应有 2 条。
/// 当最后刚好剩 1 条时，从前一组借 1 条，避免 `10 + 1` 这种尾部单条发送。
pub(super) fn album_chunk_sizes(item_count: usize) -> Vec<usize> {
    if item_count == 0 {
        return Vec::new();
    }
    if item_count <= TELEGRAM_ALBUM_MAX_ITEMS {
        return vec![item_count];
    }

    let mut sizes = Vec::new();
    let mut remaining = item_count;
    while remaining > TELEGRAM_ALBUM_MAX_ITEMS {
        let next_remaining = remaining - TELEGRAM_ALBUM_MAX_ITEMS;
        if next_remaining == 1 {
            sizes.push(TELEGRAM_ALBUM_MAX_ITEMS - 1);
            remaining -= TELEGRAM_ALBUM_MAX_ITEMS - 1;
        } else {
            sizes.push(TELEGRAM_ALBUM_MAX_ITEMS);
            remaining -= TELEGRAM_ALBUM_MAX_ITEMS;
        }
    }
    sizes.push(remaining);
    sizes
}

/// 校验多条消息是否可以按 album 发送。
///
/// 这一步只做 TDLib album 组合规则的前置检查，不负责真正上传：
/// - 单条消息不会走 album，直接允许，由上层使用 `send_message`。
/// - 超过 10 条时上传阶段会分成多个 album，这里只校验每条是否适合进入 album。
/// - 多条纯文本不能组成 album，因为 `send_message_album` 不支持文本项。
/// - 多条语音不能组成 album，因为 voice note 不能放进 Telegram album。
/// - 多条 GIF/animation 不能组成 album，因为 animation 只能单条发送。
/// - document album 必须全部都是 document，不能和 photo/video/audio 混合。
/// - audio album 必须全部都是 audio，不能和 photo/video/document 混合。
/// - photo 和 video 可以混合组成 album，所以不需要额外拦截。
pub(super) fn validate_album_kinds(kinds: &[UploadKind]) -> anyhow::Result<()> {
    // 单条内容由上层走 send_message，不需要受 album 规则限制。
    if kinds.len() <= 1 {
        return Ok(());
    }

    // Telegram album 不能包含纯文本项；文本转存需要走单条消息发送。
    if kinds.iter().any(|k| matches!(k, UploadKind::Text)) {
        anyhow::bail!("album upload doesn't support text item");
    }

    // animation/GIF 不支持放入 album；多条 GIF 当前直接失败，避免静默降级成逐条发送。
    if kinds.iter().any(|k| matches!(k, UploadKind::Animation)) {
        anyhow::bail!("album upload doesn't support animation item");
    }

    // voice note 不支持放入 album；多条语音不能用 send_message_album 合并发送。
    if kinds.iter().any(|k| matches!(k, UploadKind::Voice)) {
        anyhow::bail!("album upload doesn't support voice note item");
    }

    // document album 必须是纯 document 组，混入图片/视频/音频会被 TDLib 拒绝。
    let has_document = kinds.iter().any(|k| matches!(k, UploadKind::Document));
    if has_document && !kinds.iter().all(|k| matches!(k, UploadKind::Document)) {
        anyhow::bail!("document album requires all items to be document");
    }

    // audio album 必须是纯 audio 组，不能和其他媒体类型混合。
    let has_audio = kinds.iter().any(|k| matches!(k, UploadKind::Audio));
    if has_audio && !kinds.iter().all(|k| matches!(k, UploadKind::Audio)) {
        anyhow::bail!("audio album requires all items to be audio");
    }

    // 剩余组合当前允许，主要是 photo/video 混合 album。
    Ok(())
}
