// 上传阶段：
// - 单条消息使用 send_message
// - 多条消息使用 send_message_album 并按 TDLib 限制分批
// - 上传成功后生成结果入口链接

use crate::tgbot::TdError;

use super::super::file::{PreparedUpload, UploadKind};
use super::super::store;

/// TDLib 内部消息 ID 到 Telegram 链接可见消息 ID 的位移。
///
/// Telegram 的 `t.me/c/<chat>/<message>` 使用客户端可见的服务端消息序号；
/// TDLib `Message.id` 是内部 ID，通常把服务端消息序号左移 20 位保存。
/// 只有在 `getMessageLink` 不可用时才使用这个换算做 supergroup/channel 兜底。
const TDLIB_MESSAGE_ID_SHIFT: u32 = 20;

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
        let message = crate::tgbot::send::wait_for_sent_message(message).await?;
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
            let msg = crate::tgbot::send::wait_for_sent_message(msg.clone()).await?;
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
            if crate::tgbot::send::is_openable_url(&link.link) {
                Ok(link.link)
            } else {
                tracing::warn!(
                    chat_id,
                    message_id,
                    link = %link.link,
                    "get_message_link returned non-openable link"
                );
                if let Some(link) =
                    build_private_supergroup_message_link_from_chat(chat_id, message_id, client_id)
                        .await
                {
                    return Ok(link);
                }
                Ok(fallback_result_message_locator(chat_id, message_id))
            }
        }
        Err(err) => {
            // 上传已经成功时，链接生成失败不能反向把任务判成失败。
            tracing::warn!(
                "get_message_link failed, try private supergroup link fallback, chat_id={}, message_id={}, error={:?}",
                chat_id,
                message_id,
                err
            );
            if let Some(link) =
                build_private_supergroup_message_link_from_chat(chat_id, message_id, client_id)
                    .await
            {
                return Ok(link);
            }

            // 普通私聊/basic group 没有稳定可点击消息链接，因此只保存可复制的定位信息。
            Ok(fallback_result_message_locator(chat_id, message_id))
        }
    }
}

/// 刷新数据库中已经保存的结果链接。
///
/// 旧版本可能保存过不可点击的 `tg://openmessage` 或纯定位字符串；重复转存
/// 和 `/lookup` 命中历史成功任务时会调用这里，尽量用当前 TDLib 状态修复成
/// 可点击的 HTTPS 链接，并写回数据库供后续直接复用。
pub(in crate::tgbot::transfer) async fn refresh_stored_result_link(
    job_id: i64,
    target_chat_id: i64,
    result_message_id: Option<i64>,
    current_link: &str,
    client_id: i32,
) -> anyhow::Result<String> {
    if crate::tgbot::send::is_openable_url(current_link) {
        return Ok(current_link.to_owned());
    }

    let Some(result_message_id) =
        result_message_id.or_else(|| extract_tdlib_message_id_from_stored_link(current_link))
    else {
        tracing::warn!(
            job_id,
            target_chat_id,
            result_link = %current_link,
            "stored result link isn't openable and result_message_id is missing"
        );
        return Ok(current_link.to_owned());
    };

    // 历史任务未保存“是否相册”，刷新时用单条消息链接即可；相册首条消息同样能把用户带到结果位置。
    let refreshed =
        build_result_message_link(target_chat_id, result_message_id, false, client_id).await?;
    if refreshed == current_link {
        return Ok(refreshed);
    }

    if let Err(err) = store::update_result_message_link(job_id, refreshed.clone()).await {
        // 链接已经刷新成功，数据库写回失败不应阻止本次回复用户。
        tracing::warn!(
            job_id,
            target_chat_id,
            error = %err,
            "refresh result link succeeded but database update failed"
        );
    } else {
        tracing::info!(job_id, target_chat_id, "stored result link refreshed");
    }
    Ok(refreshed)
}

/// 构造结果消息的兜底定位。
///
/// 注意：TDLib 的 `message_id` 是 TDLib 内部消息 ID，不能随意拼成 t.me 链接。
/// 只有 supergroup/channel 兜底会先换算为可见消息 ID；其他 chat 只能保存排查用定位。
pub(super) fn fallback_result_message_locator(chat_id: i64, message_id: i64) -> String {
    format!("chat_id={} message_id={}", chat_id, message_id)
}

/// 从历史保存的旧链接或定位字符串中提取 TDLib message_id。
///
/// 兼容两类旧数据：
/// - `tg://openmessage?chat_id=...&message_id=...`
/// - `chat_id=... message_id=...`
pub(super) fn extract_tdlib_message_id_from_stored_link(link: &str) -> Option<i64> {
    link.split(['?', '&', ' '])
        .find_map(|part| part.strip_prefix("message_id=")?.parse::<i64>().ok())
}

/// 从 TDLib chat 信息构造私有 supergroup/channel 的 `t.me/c` 兜底链接。
///
/// `getMessageLink` 是首选路径；只有它失败时才进入这里。basic group、私聊、
/// secret chat 都没有稳定的 `t.me/c` 链接，因此会返回 None，调用方再保存定位信息。
async fn build_private_supergroup_message_link_from_chat(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Option<String> {
    let chat = match tdlib_rs::functions::get_chat(chat_id, client_id).await {
        Ok(chat) => chat,
        Err(err) => {
            tracing::warn!(
                chat_id,
                message_id,
                error = ?err,
                "get_chat failed while building private supergroup message link"
            );
            return None;
        }
    };
    let tdlib_rs::enums::Chat::Chat(chat) = chat;
    match chat.r#type {
        tdlib_rs::enums::ChatType::Supergroup(supergroup) => {
            build_private_supergroup_message_link(supergroup.supergroup_id, message_id)
        }
        other => {
            tracing::debug!(
                chat_id,
                message_id,
                chat_type = ?other,
                "chat type doesn't support private t.me/c message link fallback"
            );
            None
        }
    }
}

/// 构造 `https://t.me/c/<supergroup_id>/<message_id>` 私有群/频道消息链接。
///
/// 这里的 `message_id` 必须先从 TDLib 内部 ID 换算为 Telegram 链接里的可见 ID，
/// 否则会出现旧版本那种“链接可点击但无法跳转到消息”的问题。
pub(super) fn build_private_supergroup_message_link(
    supergroup_id: i64,
    tdlib_message_id: i64,
) -> Option<String> {
    let visible_message_id = tdlib_message_id_to_visible_id(tdlib_message_id)?;
    Some(format!(
        "https://t.me/c/{}/{}",
        supergroup_id, visible_message_id
    ))
}

/// 把 TDLib 内部消息 ID 换算成 Telegram 链接使用的可见消息 ID。
///
/// 如果换算结果不是正数，说明该 ID 不是正常的已发送消息 ID，不能构造链接。
pub(super) fn tdlib_message_id_to_visible_id(tdlib_message_id: i64) -> Option<i64> {
    let visible_message_id = tdlib_message_id >> TDLIB_MESSAGE_ID_SHIFT;
    (visible_message_id > 0).then_some(visible_message_id)
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
