//!
//! TDLib `forum` domain functions.
//!
//! Types, enums, and functions for supergroup forum topics and topic management.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Loads more topics in a channel direct messages chat administered by the current user. The loaded topics will be sent through updateDirectMessagesChatTopic.
/// Topics are sorted by their topic.order in descending order. Returns a 404 error if all topics have been loaded
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `limit` - The maximum number of topics to be loaded. For optimal performance, the number of loaded topics is chosen by TDLib and can be smaller than the specified limit, even if the end of the list is not reached
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_direct_messages_chat_topics(chat_id: i64, limit: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadDirectMessagesChatTopics",
        "chat_id": chat_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about the topic in a channel direct messages chat administered by the current user
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic to get
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::DirectMessagesChatTopic)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_direct_messages_chat_topic(chat_id: i64, topic_id: i64, client_id: i32) -> Result<crate::enums::DirectMessagesChatTopic, crate::types::Error> {
    let request = json!({
        "@type": "getDirectMessagesChatTopic",
        "chat_id": chat_id,
        "topic_id": topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the last message sent in the topic in a channel direct messages chat administered by the current user no later than the specified date
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic which messages will be fetched
/// * `date` - Point in time (Unix timestamp) relative to which to search for messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_direct_messages_chat_topic_message_by_date(chat_id: i64, topic_id: i64, date: i32, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getDirectMessagesChatTopicMessageByDate",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "date": date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes all messages between the specified dates in the topic in a channel direct messages chat administered by the current user. Messages sent in the last 30 seconds will not be deleted
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic which messages will be deleted
/// * `min_date` - The minimum date of the messages to delete
/// * `max_date` - The maximum date of the messages to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_direct_messages_chat_topic_messages_by_date(chat_id: i64, topic_id: i64, min_date: i32, max_date: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteDirectMessagesChatTopicMessagesByDate",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "min_date": min_date,
        "max_date": max_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the marked as unread state of the topic in a channel direct messages chat administered by the current user
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Topic identifier
/// * `is_marked_as_unread` - New value of is_marked_as_unread
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_direct_messages_chat_topic_is_marked_as_unread(chat_id: i64, topic_id: i64, is_marked_as_unread: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDirectMessagesChatTopicIsMarkedAsUnread",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "is_marked_as_unread": is_marked_as_unread,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes all pinned messages from the topic in a channel direct messages chat administered by the current user
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `topic_id` - Topic identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn unpin_all_direct_messages_chat_topic_messages(chat_id: i64, topic_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "unpinAllDirectMessagesChatTopicMessages",
        "chat_id": chat_id,
        "topic_id": topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes all unread reactions in the topic in a channel direct messages chat administered by the current user
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `topic_id` - Topic identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_all_direct_messages_chat_topic_reactions(chat_id: i64, topic_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllDirectMessagesChatTopicReactions",
        "chat_id": chat_id,
        "topic_id": topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Allows to send unpaid messages to the given topic of the channel direct messages chat administered by the current user
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `topic_id` - Identifier of the topic
/// * `can_send_unpaid_messages` - Pass true to allow unpaid messages; pass false to disallow unpaid messages
/// * `refund_payments` - Pass true to refund the user previously paid messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_direct_messages_chat_topic_can_send_unpaid_messages(chat_id: i64, topic_id: i64, can_send_unpaid_messages: bool, refund_payments: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleDirectMessagesChatTopicCanSendUnpaidMessages",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "can_send_unpaid_messages": can_send_unpaid_messages,
        "refund_payments": refund_payments,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Loads more Saved Messages topics. The loaded topics will be sent through updateSavedMessagesTopic. Topics are sorted by their topic.order in descending order. Returns a 404 error if all topics have been loaded
///
/// # Arguments
///
/// * `limit` - The maximum number of topics to be loaded. For optimal performance, the number of loaded topics is chosen by TDLib and can be smaller than the specified limit, even if the end of the list is not reached
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_saved_messages_topics(limit: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadSavedMessagesTopics",
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the last message sent in a Saved Messages topic no later than the specified date
///
/// # Arguments
///
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which message will be returned
/// * `date` - Point in time (Unix timestamp) relative to which to search for messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_messages_topic_message_by_date(saved_messages_topic_id: i64, date: i32, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getSavedMessagesTopicMessageByDate",
        "saved_messages_topic_id": saved_messages_topic_id,
        "date": date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes all messages between the specified dates in a Saved Messages topic. Messages sent in the last 30 seconds will not be deleted
///
/// # Arguments
///
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which messages will be deleted
/// * `min_date` - The minimum date of the messages to delete
/// * `max_date` - The maximum date of the messages to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_saved_messages_topic_messages_by_date(saved_messages_topic_id: i64, min_date: i32, max_date: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteSavedMessagesTopicMessagesByDate",
        "saved_messages_topic_id": saved_messages_topic_id,
        "min_date": min_date,
        "max_date": max_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the pinned state of a Saved Messages topic. There can be up to getOption("pinned_saved_messages_topic_count_max") pinned topics. The limit can be increased with Telegram Premium
///
/// # Arguments
///
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic to pin or unpin
/// * `is_pinned` - Pass true to pin the topic; pass false to unpin it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_saved_messages_topic_is_pinned(saved_messages_topic_id: i64, is_pinned: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSavedMessagesTopicIsPinned",
        "saved_messages_topic_id": saved_messages_topic_id,
        "is_pinned": is_pinned,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the order of pinned Saved Messages topics
///
/// # Arguments
///
/// * `saved_messages_topic_ids` - Identifiers of the new pinned Saved Messages topics
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_saved_messages_topics(saved_messages_topic_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPinnedSavedMessagesTopics",
        "saved_messages_topic_ids": saved_messages_topic_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of custom emoji, which can be used as forum topic icon by all users
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stickers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topic_default_icons(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getForumTopicDefaultIcons",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a topic in a forum supergroup chat or a chat with a bot with topics; requires can_manage_topics administrator or can_create_topics member right in the supergroup
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `name` - Name of the topic; 1-128 characters
/// * `is_name_implicit` - Pass true if the name of the topic wasn't entered explicitly; for chats with bots only
/// * `icon` - Icon of the topic. Icon color must be one of 0x6FB9F0, 0xFFD67E, 0xCB86DB, 0x8EEE98, 0xFF93B2, or 0xFB6F5F. Telegram Premium users can use any custom emoji as topic icon, other users can use only a custom emoji returned by getForumTopicDefaultIcons
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ForumTopicInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_forum_topic(chat_id: i64, name: String, is_name_implicit: bool, icon: crate::types::ForumTopicIcon, client_id: i32) -> Result<crate::enums::ForumTopicInfo, crate::types::Error> {
    let request = json!({
        "@type": "createForumTopic",
        "chat_id": chat_id,
        "name": name,
        "is_name_implicit": is_name_implicit,
        "icon": icon,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits title and icon of a topic in a forum supergroup chat or a chat with a bot with topics; for supergroup chats requires can_manage_topics administrator right
/// unless the user is creator of the topic
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `name` - New name of the topic; 0-128 characters. If empty, the previous topic name is kept
/// * `edit_icon_custom_emoji` - Pass true to edit the icon of the topic. Icon of the General topic can't be edited
/// * `icon_custom_emoji_id` - Identifier of the new custom emoji for topic icon; pass 0 to remove the custom emoji. Ignored if edit_icon_custom_emoji is false. Telegram Premium users can use any custom emoji, other users can use only a custom emoji returned by getForumTopicDefaultIcons
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_forum_topic(chat_id: i64, forum_topic_id: i32, name: String, edit_icon_custom_emoji: bool, icon_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editForumTopic",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "name": name,
        "edit_icon_custom_emoji": edit_icon_custom_emoji,
        "icon_custom_emoji_id": icon_custom_emoji_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a topic in a forum supergroup chat or a chat with a bot with topics
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ForumTopic)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topic(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<crate::enums::ForumTopic, crate::types::Error> {
    let request = json!({
        "@type": "getForumTopic",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTPS link to a topic in a forum supergroup chat. This is an offline method
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topic_link(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<crate::enums::MessageLink, crate::types::Error> {
    let request = json!({
        "@type": "getForumTopicLink",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns found forum topics in a forum supergroup chat or a chat with a bot with topics. This is a temporary method for getting information about topic list from the server
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `query` - Query to search for in the forum topic's name
/// * `offset_date` - The date starting from which the results need to be fetched. Use 0 or any date in the future to get results from the last topic
/// * `offset_message_id` - The message identifier of the last message in the last found topic, or 0 for the first request
/// * `offset_forum_topic_id` - The forum topic identifier of the last found topic, or 0 for the first request
/// * `limit` - The maximum number of forum topics to be returned; up to 100. For optimal performance, the number of returned forum topics is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ForumTopics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topics(chat_id: i64, query: String, offset_date: i32, offset_message_id: i64, offset_forum_topic_id: i32, limit: i32, client_id: i32) -> Result<crate::enums::ForumTopics, crate::types::Error> {
    let request = json!({
        "@type": "getForumTopics",
        "chat_id": chat_id,
        "query": query,
        "offset_date": offset_date,
        "offset_message_id": offset_message_id,
        "offset_forum_topic_id": offset_forum_topic_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the notification settings of a forum topic in a forum supergroup chat or a chat with a bot with topics
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier
/// * `notification_settings` - New notification settings for the forum topic. If the topic is muted for more than 366 days, it is considered to be muted forever
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_forum_topic_notification_settings(chat_id: i64, forum_topic_id: i32, notification_settings: crate::types::ChatNotificationSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setForumTopicNotificationSettings",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "notification_settings": notification_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether a topic is closed in a forum supergroup chat; requires can_manage_topics administrator right in the supergroup unless the user is creator of the topic
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `is_closed` - Pass true to close the topic; pass false to reopen it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_forum_topic_is_closed(chat_id: i64, forum_topic_id: i32, is_closed: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleForumTopicIsClosed",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "is_closed": is_closed,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether a General topic is hidden in a forum supergroup chat; requires can_manage_topics administrator right in the supergroup
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `is_hidden` - Pass true to hide and close the General topic; pass false to unhide it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_general_forum_topic_is_hidden(chat_id: i64, is_hidden: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGeneralForumTopicIsHidden",
        "chat_id": chat_id,
        "is_hidden": is_hidden,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the pinned state of a topic in a forum supergroup chat or a chat with a bot with topics; requires can_manage_topics administrator right in the supergroup.
/// There can be up to getOption("pinned_forum_topic_count_max") pinned forum topics
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier
/// * `is_pinned` - Pass true to pin the topic; pass false to unpin it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_forum_topic_is_pinned(chat_id: i64, forum_topic_id: i32, is_pinned: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleForumTopicIsPinned",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "is_pinned": is_pinned,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the order of pinned topics in a forum supergroup chat or a chat with a bot with topics; requires can_manage_topics administrator right in the supergroup
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_ids` - The new list of identifiers of the pinned forum topics
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_forum_topics(chat_id: i64, forum_topic_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPinnedForumTopics",
        "chat_id": chat_id,
        "forum_topic_ids": forum_topic_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all messages from a topic in a forum supergroup chat or a chat with a bot with topics; requires can_delete_messages administrator right in the supergroup
/// unless the user is creator of the topic, the topic has no messages from other users and has at most 11 messages
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_forum_topic(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteForumTopic",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Marks all mentions in a topic in a forum supergroup chat as read
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier in which mentions are marked as read
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_all_forum_topic_mentions(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllForumTopicMentions",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Marks all reactions in a topic in a forum supergroup chat or a chat with a bot with topics as read
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier in which reactions are marked as read
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_all_forum_topic_reactions(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllForumTopicReactions",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Marks all poll votes in a topic in a forum supergroup chat as read
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier in which poll votes are marked as read
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_all_forum_topic_poll_votes(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllForumTopicPollVotes",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes all pinned messages from a topic in a forum supergroup chat or a chat with a bot with topics; requires can_pin_messages member right in the supergroup
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier in which messages will be unpinned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn unpin_all_forum_topic_messages(chat_id: i64, forum_topic_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "unpinAllForumTopicMessages",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the view_as_topics setting of a forum chat or Saved Messages
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `view_as_topics` - New value of view_as_topics
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_view_as_topics(chat_id: i64, view_as_topics: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleChatViewAsTopics",
        "chat_id": chat_id,
        "view_as_topics": view_as_topics,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether the supergroup is a forum; requires owner privileges in the supergroup. Discussion supergroups can't be converted to forums
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup
/// * `is_forum` - New value of is_forum
/// * `has_forum_tabs` - New value of has_forum_tabs; ignored if is_forum is false
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_is_forum(supergroup_id: i64, is_forum: bool, has_forum_tabs: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupIsForum",
        "supergroup_id": supergroup_id,
        "is_forum": is_forum,
        "has_forum_tabs": has_forum_tabs,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

