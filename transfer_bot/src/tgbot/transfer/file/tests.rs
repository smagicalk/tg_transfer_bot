// 文件识别相关单元测试。
// 这里只构造 TDLib 类型，不触发真实 TDLib 下载。

use super::*;

/// 构造测试用 TDLib File，避免依赖真实 TDLib 下载。
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

/// 构造最小可用的 Message，测试只关心 content/chat_id/id。
fn message_with_content(content: tdlib_rs::enums::MessageContent) -> tdlib_rs::types::Message {
    tdlib_rs::types::Message {
        id: 200,
        sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
            user_id: 1,
        }),
        chat_id: 100,
        sending_state: None,
        scheduling_state: None,
        is_outgoing: false,
        is_pinned: false,
        is_from_offline: false,
        can_be_saved: true,
        has_timestamped_media: false,
        is_channel_post: false,
        is_paid_star_suggested_post: false,
        is_paid_ton_suggested_post: false,
        contains_unread_mention: false,
        date: 0,
        edit_date: 0,
        forward_info: None,
        import_info: None,
        interaction_info: None,
        unread_reactions: vec![],
        fact_check: None,
        suggested_post_info: None,
        reply_to: None,
        topic_id: None,
        self_destruct_type: None,
        self_destruct_in: 0.0,
        auto_delete_in: 0.0,
        via_bot_user_id: 0,
        sender_business_bot_user_id: 0,
        sender_boost_count: 0,
        sender_tag: String::new(),
        paid_message_star_count: 0,
        author_signature: String::new(),
        media_album_id: 0,
        effect_id: 0,
        restriction_info: None,
        summary_language_code: String::new(),
        content,
        reply_markup: None,
    }
}

/// 语音消息应能提取稳定 file_key，后续才能参与文件缓存与下载去重。
#[test]
fn test_extract_file_key_supports_voice_note() {
    let message = message_with_content(tdlib_rs::enums::MessageContent::MessageVoiceNote(
        tdlib_rs::types::MessageVoiceNote {
            voice_note: tdlib_rs::types::VoiceNote {
                voice: test_file(),
                ..Default::default()
            },
            caption: tdlib_rs::types::FormattedText::default(),
            is_listened: false,
        },
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
        tdlib_rs::types::MessageVoiceNote {
            voice_note: tdlib_rs::types::VoiceNote {
                voice: test_file(),
                ..Default::default()
            },
            caption: tdlib_rs::types::FormattedText::default(),
            is_listened: false,
        },
    ));

    let seed = extract_download_seed(&message).expect("voice note should have seed");
    assert_eq!(seed.file_key, "voice_unique_key");
    assert_eq!(seed.td_file_id, 42);
    assert_eq!(seed.size_bytes, Some(1024));
}
