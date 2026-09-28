// 文件识别相关单元测试。
// 这里只构造 TDLib 类型，不触发真实 TDLib 下载。

use super::*;

/// 构造测试用 TDLib File，避免依赖真实 TDLib 下载。
///
/// 设置测试所需的 `id`、`size` 以及远程文件的 `unique_id`。
fn test_file() -> tdlib_rs::types::File {
    tdlib_rs::types::File {
        id: 42,
        size: 1024,
        expected_size: 2048,
        remote: tdlib_rs::types::RemoteFile {
            unique_id: "voice_unique_key".to_owned(),
            id: "remote_id".to_owned(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// 构造测试用动画（Animation / GIF）结构。
///
/// # 返回值
/// - 包含宽高、时长及测试文件的 TDLib `Animation` 实例。
fn test_animation() -> tdlib_rs::types::Animation {
    tdlib_rs::types::Animation {
        duration: 1,
        width: 320,
        height: 180,
        file_name: "test.gif".to_owned(),
        mime_type: "image/gif".to_owned(),
        has_stickers: false,
        minithumbnail: None,
        thumbnail: None,
        animation: test_file(),
    }
}

/// 构造测试用贴纸（Sticker）结构。
///
/// # 返回值
/// - 包含 emoji、尺寸和测试文件的 TDLib `Sticker` 实例。
fn test_sticker() -> tdlib_rs::types::Sticker {
    tdlib_rs::types::Sticker {
        id: 1,
        set_id: 0,
        width: 128,
        height: 128,
        emoji: "x".to_owned(),
        format: tdlib_rs::enums::StickerFormat::Webp,
        full_type: tdlib_rs::enums::StickerFullType::Regular(Box::default()),
        thumbnail: None,
        sticker: test_file(),
    }
}

/// 构造最小可用的 TDLib Message 实体，便于测试各文件类型的解析与过滤。
///
/// # 参数
/// - `content`: 要注入到消息中的具体消息内容（如 Photo、Video、Animation 等）。
///
/// # 返回值
/// - 预设好基础字段（id=200, chat_id=100）的 `Message` 对象。
fn message_with_content(content: tdlib_rs::enums::MessageContent) -> tdlib_rs::types::Message {
    tdlib_rs::types::Message {
        id: 200,
        sender_id: tdlib_rs::enums::MessageSender::User(Box::new(
            tdlib_rs::types::MessageSenderUser { user_id: 1 },
        )),
        chat_id: 100,
        can_be_saved: true,
        content,
        ..crate::tgbot::mock_message()
    }
}

/// GIF/animation 也应作为可转存媒体处理。
#[test]
fn test_animation_is_transferable_and_has_file_key() {
    let message = message_with_content(tdlib_rs::enums::MessageContent::MessageAnimation(
        Box::new(tdlib_rs::types::MessageAnimation {
            animation: test_animation(),
            caption: tdlib_rs::types::FormattedText::default(),
            show_caption_above_media: false,
            has_spoiler: false,
            is_secret: false,
        }),
    ));

    assert!(is_transferable_message(&message));
    assert_eq!(
        extract_file_key(&message),
        Some("voice_unique_key".to_owned())
    );
    let seed = extract_download_seed(&message).expect("animation should have seed");
    assert_eq!(seed.td_file_id, 42);
}

/// 暂不支持的消息类型不能自动进入转存队列。
#[test]
fn test_sticker_is_not_transferable() {
    let message = message_with_content(tdlib_rs::enums::MessageContent::MessageSticker(Box::new(
        tdlib_rs::types::MessageSticker {
            sticker: test_sticker(),
            is_premium: false,
        },
    )));

    assert!(!is_transferable_message(&message));
}

/// 语音消息应能提取稳定 file_key，后续才能参与文件缓存与下载去重。
#[test]
fn test_extract_file_key_supports_voice_note() {
    let message = message_with_content(tdlib_rs::enums::MessageContent::MessageVoiceNote(
        Box::new(tdlib_rs::types::MessageVoiceNote {
            voice_note: tdlib_rs::types::VoiceNote {
                voice: test_file(),
                ..Default::default()
            },
            caption: tdlib_rs::types::FormattedText::default(),
            is_listened: false,
        }),
    ));

    assert_eq!(
        extract_file_key(&message),
        Some("voice_unique_key".to_owned())
    );
}

/// 语音消息应能生成下载种子，进度查询才能拿到 td_file_id 与大小。
#[test]
fn test_extract_download_seed_supports_voice_note() {
    let message = message_with_content(tdlib_rs::enums::MessageContent::MessageVoiceNote(
        Box::new(tdlib_rs::types::MessageVoiceNote {
            voice_note: tdlib_rs::types::VoiceNote {
                voice: test_file(),
                ..Default::default()
            },
            caption: tdlib_rs::types::FormattedText::default(),
            is_listened: false,
        }),
    ));

    let seed = extract_download_seed(&message).expect("voice note should have seed");
    assert_eq!(seed.file_key, "voice_unique_key");
    assert_eq!(seed.td_file_id, 42);
    assert_eq!(seed.size_bytes, Some(1024));
}
