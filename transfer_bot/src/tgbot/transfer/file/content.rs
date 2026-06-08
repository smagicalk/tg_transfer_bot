// 上传内容构建逻辑。
// 这里负责把 TDLib MessageContent 转成可发送的 InputMessageContent。

use super::download::prepare_media_file;
use super::types::{PreparedUpload, UploadKind};

/// 按消息类型准备上传内容：
/// - 媒体消息：确保文件本地可用，并生成 InputFile::Local
/// - 文本消息：直接转换为 InputMessageText
pub(in crate::tgbot::transfer) async fn prepare_upload_content(
    message: &tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<PreparedUpload> {
    match &message.content {
        tdlib_rs::enums::MessageContent::MessagePhoto(photo) => {
            let best = photo
                .photo
                .sizes
                .iter()
                .max_by_key(|s| (s.width as i64) * (s.height as i64))
                .ok_or_else(|| anyhow::anyhow!("photo has no available size"))?;

            let (prepared_file, local_input) = prepare_media_file(&best.photo, client_id).await?;
            let content = tdlib_rs::enums::InputMessageContent::InputMessagePhoto(
                tdlib_rs::types::InputMessagePhoto {
                    photo: local_input,
                    thumbnail: None,
                    added_sticker_file_ids: vec![],
                    width: best.width,
                    height: best.height,
                    caption: to_optional_caption(&photo.caption),
                    // 相册发送要求同一组 show_caption_above_media 一致，这里统一为 false。
                    show_caption_above_media: false,
                    self_destruct_type: None,
                    has_spoiler: photo.has_spoiler,
                },
            );

            Ok(PreparedUpload {
                input_content: content,
                kind: UploadKind::Photo,
                cache_meta: Some(prepared_file),
            })
        }
        tdlib_rs::enums::MessageContent::MessageVideo(video) => {
            let (prepared_file, local_input) =
                prepare_media_file(&video.video.video, client_id).await?;
            let content = tdlib_rs::enums::InputMessageContent::InputMessageVideo(
                tdlib_rs::types::InputMessageVideo {
                    video: local_input,
                    thumbnail: None,
                    cover: None,
                    start_timestamp: video.start_timestamp,
                    added_sticker_file_ids: vec![],
                    duration: video.video.duration,
                    width: video.video.width,
                    height: video.video.height,
                    supports_streaming: video.video.supports_streaming,
                    caption: to_optional_caption(&video.caption),
                    // 相册发送要求同一组 show_caption_above_media 一致，这里统一为 false。
                    show_caption_above_media: false,
                    self_destruct_type: None,
                    has_spoiler: video.has_spoiler,
                },
            );

            Ok(PreparedUpload {
                input_content: content,
                kind: UploadKind::Video,
                cache_meta: Some(prepared_file),
            })
        }
        tdlib_rs::enums::MessageContent::MessageDocument(document) => {
            let (prepared_file, local_input) =
                prepare_media_file(&document.document.document, client_id).await?;
            let content = tdlib_rs::enums::InputMessageContent::InputMessageDocument(
                tdlib_rs::types::InputMessageDocument {
                    document: local_input,
                    thumbnail: None,
                    disable_content_type_detection: false,
                    caption: to_optional_caption(&document.caption),
                },
            );

            Ok(PreparedUpload {
                input_content: content,
                kind: UploadKind::Document,
                cache_meta: Some(prepared_file),
            })
        }
        tdlib_rs::enums::MessageContent::MessageAudio(audio) => {
            let (prepared_file, local_input) =
                prepare_media_file(&audio.audio.audio, client_id).await?;
            let content = tdlib_rs::enums::InputMessageContent::InputMessageAudio(
                tdlib_rs::types::InputMessageAudio {
                    audio: local_input,
                    album_cover_thumbnail: None,
                    duration: audio.audio.duration,
                    title: audio.audio.title.clone(),
                    performer: audio.audio.performer.clone(),
                    caption: to_optional_caption(&audio.caption),
                },
            );

            Ok(PreparedUpload {
                input_content: content,
                kind: UploadKind::Audio,
                cache_meta: Some(prepared_file),
            })
        }
        tdlib_rs::enums::MessageContent::MessageVoiceNote(voice) => {
            let (prepared_file, local_input) =
                prepare_media_file(&voice.voice_note.voice, client_id).await?;
            let content = tdlib_rs::enums::InputMessageContent::InputMessageVoiceNote(
                tdlib_rs::types::InputMessageVoiceNote {
                    voice_note: local_input,
                    duration: voice.voice_note.duration,
                    waveform: voice.voice_note.waveform.clone(),
                    caption: to_optional_caption(&voice.caption),
                    // 转存普通语音不继承阅后即焚属性，避免目标聊天出现不可预期的自毁消息。
                    self_destruct_type: None,
                },
            );

            Ok(PreparedUpload {
                input_content: content,
                kind: UploadKind::Voice,
                cache_meta: Some(prepared_file),
            })
        }
        tdlib_rs::enums::MessageContent::MessageText(text) => {
            let content = tdlib_rs::enums::InputMessageContent::InputMessageText(
                tdlib_rs::types::InputMessageText {
                    text: text.text.clone(),
                    link_preview_options: text.link_preview_options.clone(),
                    clear_draft: false,
                },
            );

            Ok(PreparedUpload {
                input_content: content,
                kind: UploadKind::Text,
                cache_meta: None,
            })
        }
        _ => anyhow::bail!("unsupported message content for transfer upload"),
    }
}

/// 将 TDLib FormattedText 转为可选 caption：
/// 空文本转换为 None，避免发送空 caption。
fn to_optional_caption(
    text: &tdlib_rs::types::FormattedText,
) -> Option<tdlib_rs::types::FormattedText> {
    if text.text.trim().is_empty() {
        None
    } else {
        Some(text.clone())
    }
}
