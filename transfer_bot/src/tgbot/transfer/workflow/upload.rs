// 上传阶段：
// - 单条消息使用 send_message
// - 多条消息使用 send_message_album 并按 TDLib 限制分批
// - 上传成功后生成结果入口链接

use crate::tgbot::TdError;

use super::super::file::{PreparedUpload, UploadKind};

/// 上传准备好的消息集合：
/// - 1 条 => send_message
/// - 多条 => send_message_album（按 10 条分批）
pub(super) async fn upload_prepared(
    target_chat_id: i64,
    prepared: &[(i64, PreparedUpload)],
    client_id: i32,
) -> anyhow::Result<UploadResult> {
    if prepared.is_empty() {
        anyhow::bail!("no prepared item to upload");
    }

    if prepared.len() == 1 {
        let content = prepared[0].1.input_content.clone();
        let sent =
            tdlib_rs::functions::send_message(target_chat_id, None, None, None, content, client_id)
                .await
                .map_err(|e| anyhow::Error::new(TdError(e)))?;
        let tdlib_rs::enums::Message::Message(message) = sent;
        return Ok(UploadResult {
            result_message_id: message.id,
            is_album: false,
        });
    }

    let kinds = prepared.iter().map(|(_, p)| p.kind).collect::<Vec<_>>();
    validate_album_kinds(&kinds)?;

    // TDLib 限制每个 album 最多 10 条，超出时拆成多个 album 顺序发送。
    let contents = prepared
        .iter()
        .map(|(_, p)| p.input_content.clone())
        .collect::<Vec<_>>();
    let mut first_message_id: Option<i64> = None;
    for chunk in contents.chunks(10) {
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
        if first_message_id.is_none()
            && let Some(Some(msg)) = messages.messages.first()
        {
            first_message_id = Some(msg.id);
        }
    }
    let result_message_id = first_message_id
        .ok_or_else(|| anyhow::anyhow!("send_message_album returned no message id"))?;
    Ok(UploadResult {
        result_message_id,
        is_album: true,
    })
}

/// 上传结果摘要。
#[derive(Debug, Clone, Copy)]
pub(super) struct UploadResult {
    pub result_message_id: i64,
    pub is_album: bool,
}

/// 为上传结果构建入口消息链接。
pub(super) async fn build_result_message_link(
    chat_id: i64,
    message_id: i64,
    is_album: bool,
    client_id: i32,
) -> anyhow::Result<String> {
    let rs =
        tdlib_rs::functions::get_message_link(chat_id, message_id, 0, is_album, false, client_id)
            .await;

    match rs {
        Ok(rs) => {
            let tdlib_rs::enums::MessageLink::MessageLink(link) = rs;
            Ok(link.link)
        }
        Err(err) => {
            // 上传已经成功时，链接生成失败不能反向把任务判成失败；这里保留可定位消息的兜底链接。
            tracing::warn!(
                "get_message_link failed, use fallback link, chat_id={}, message_id={}, error={:?}",
                chat_id,
                message_id,
                err
            );
            Ok(fallback_result_message_link(chat_id, message_id))
        }
    }
}

/// 构造结果消息的兜底链接。
///
/// `-100...` 形式的群/频道使用 t.me/c 链接；其他 chat 使用 Telegram 客户端 deeplink，
/// 保证上传成功后数据库至少有一个可定位目标消息的值。
pub(super) fn fallback_result_message_link(chat_id: i64, message_id: i64) -> String {
    const CHANNEL_CHAT_ID_PREFIX: i64 = 1_000_000_000_000;
    let abs_chat_id = chat_id.saturating_abs();
    if chat_id < 0 && abs_chat_id > CHANNEL_CHAT_ID_PREFIX {
        let internal_id = abs_chat_id - CHANNEL_CHAT_ID_PREFIX;
        return format!("https://t.me/c/{}/{}", internal_id, message_id);
    }

    format!(
        "tg://openmessage?chat_id={}&message_id={}",
        chat_id, message_id
    )
}

/// 校验多条消息是否可以按 album 发送。
///
/// 这一步只做 TDLib album 组合规则的前置检查，不负责真正上传：
/// - 单条消息不会走 album，直接允许，由上层使用 `send_message`。
/// - 多条纯文本不能组成 album，因为 `send_message_album` 不支持文本项。
/// - 多条语音不能组成 album，因为 voice note 不能放进 Telegram album。
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
