//!
//! TDLib `story` domain functions.
//!
//! Types, enums, and functions for Telegram Stories and story interactions.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns messages in the topic in a channel direct messages chat administered by the current user. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id)
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic which messages will be fetched
/// * `from_message_id` - Identifier of the message starting from which messages must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number from -99 to -1 to get additionally -offset newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than or equal to -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_direct_messages_chat_topic_history(chat_id: i64, topic_id: i64, from_message_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getDirectMessagesChatTopicHistory",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes all messages in the topic in a channel direct messages chat administered by the current user
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic which messages will be deleted
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_direct_messages_chat_topic_history(chat_id: i64, topic_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteDirectMessagesChatTopicHistory",
        "chat_id": chat_id,
        "topic_id": topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns messages in a Saved Messages topic. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id)
///
/// # Arguments
///
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which messages will be fetched
/// * `from_message_id` - Identifier of the message starting from which messages must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number from -99 to -1 to get additionally -offset newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than or equal to -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_messages_topic_history(saved_messages_topic_id: i64, from_message_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getSavedMessagesTopicHistory",
        "saved_messages_topic_id": saved_messages_topic_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes all messages in a Saved Messages topic
///
/// # Arguments
///
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which messages will be deleted
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_saved_messages_topic_history(saved_messages_topic_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteSavedMessagesTopicHistory",
        "saved_messages_topic_id": saved_messages_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns messages in a chat. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id).
/// For optimal performance, the number of returned messages is chosen by TDLib. This is an offline method if only_local is true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `from_message_id` - Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number from -99 to -1 to get additionally -offset newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than or equal to -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `only_local` - Pass true to get only messages that are available without sending network requests
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_history(chat_id: i64, from_message_id: i64, offset: i32, limit: i32, only_local: bool, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getChatHistory",
        "chat_id": chat_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
        "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns messages in a message thread of a message. Can be used only if messageProperties.can_get_message_thread == true. Message thread of a channel message is in the channel's linked supergroup.
/// The messages are returned in reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Message identifier, which thread history needs to be returned
/// * `from_message_id` - Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number from -99 to -1 to get additionally -offset newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than or equal to -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_thread_history(chat_id: i64, message_id: i64, from_message_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getMessageThreadHistory",
        "chat_id": chat_id,
        "message_id": message_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes all messages in the chat. Use chat.can_be_deleted_only_for_self and chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `remove_from_chat_list` - Pass true to remove the chat from all chat lists
/// * `revoke` - Pass true to delete chat history for all users
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_history(chat_id: i64, remove_from_chat_list: bool, revoke: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatHistory",
        "chat_id": chat_id,
        "remove_from_chat_list": remove_from_chat_list,
        "revoke": revoke,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches for public stories containing the given hashtag or cashtag. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the stories to search for; pass 0 to search stories in all chats
/// * `tag` - Hashtag or cashtag to search for
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of stories to be returned; up to 100. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundStories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_public_stories_by_tag(story_poster_chat_id: i64, tag: String, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundStories, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicStoriesByTag",
        "story_poster_chat_id": story_poster_chat_id,
        "tag": tag,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for public stories by the given address location. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
///
/// # Arguments
///
/// * `address` - Address of the location
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of stories to be returned; up to 100. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundStories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_public_stories_by_location(address: crate::types::LocationAddress, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundStories, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicStoriesByLocation",
        "address": address,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for public stories from the given venue. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
///
/// # Arguments
///
/// * `venue_provider` - Provider of the venue
/// * `venue_id` - Identifier of the venue in the provider database
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of stories to be returned; up to 100. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundStories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_public_stories_by_venue(venue_provider: String, venue_id: String, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundStories, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicStoriesByVenue",
        "venue_provider": venue_provider,
        "venue_id": venue_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes a story posted by the bot on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to edit
/// * `content` - New content of the story
/// * `areas` - New clickable rectangle areas to be shown on the story media
/// * `caption` - New story caption
/// * `privacy_settings` - The new privacy settings for the story
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Story)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_story(story_poster_chat_id: i64, story_id: i32, content: crate::enums::InputStoryContent, areas: crate::types::InputStoryAreas, caption: crate::types::FormattedText, privacy_settings: crate::enums::StoryPrivacySettings, client_id: i32) -> Result<crate::enums::Story, crate::types::Error> {
    let request = json!({
        "@type": "editBusinessStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "content": content,
        "areas": areas,
        "caption": caption,
        "privacy_settings": privacy_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes a story posted by the bot on behalf of a business account; for bots only
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection
/// * `story_id` - Identifier of the story to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_story(business_connection_id: String, story_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteBusinessStory",
        "business_connection_id": business_connection_id,
        "story_id": story_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns messages in a topic in a forum supergroup chat or a chat with a bot with topics. The messages are returned in reverse chronological order
/// (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier
/// * `from_message_id` - Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number from -99 to -1 to get additionally -offset newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than or equal to -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topic_history(chat_id: i64, forum_topic_id: i32, from_message_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getForumTopicHistory",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a story
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Story identifier
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Story)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_story(story_poster_chat_id: i64, story_id: i32, only_local: bool, client_id: i32) -> Result<crate::enums::Story, crate::types::Error> {
    let request = json!({
        "@type": "getStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns supergroup and channel chats in which the current user has the right to post stories. The chats must be rechecked with canPostStory before actually trying to post a story there
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chats_to_post_stories(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getChatsToPostStories",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether the current user can post a story on behalf of a chat; requires can_post_stories administrator right for supergroup and channel chats
///
/// # Arguments
///
/// * `chat_id` - Chat identifier. Pass Saved Messages chat identifier when posting a story on behalf of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CanPostStoryResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn can_post_story(chat_id: i64, client_id: i32) -> Result<crate::enums::CanPostStoryResult, crate::types::Error> {
    let request = json!({
        "@type": "canPostStory",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Posts a new story on behalf of a chat; requires can_post_stories administrator right for supergroup and channel chats. Returns a temporary story
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that will post the story. Pass Saved Messages chat identifier when posting a story on behalf of the current user
/// * `content` - Content of the story
/// * `areas` - Clickable rectangle areas to be shown on the story media; pass null if none
/// * `caption` - Story caption; pass null to use an empty caption; 0-getOption("story_caption_length_max") characters; can have entities only if getOption("can_use_text_entities_in_story_caption")
/// * `privacy_settings` - The privacy settings for the story; ignored for stories posted on behalf of supergroup and channel chats
/// * `album_ids` - Identifiers of story albums to which the story will be added upon posting. An album can have up to getOption("story_album_size_max") stories
/// * `active_period` - Period after which the story is moved to archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400 for Telegram Premium users, and 86400 otherwise
/// * `from_story_full_id` - Full identifier of the original story, which content was used to create the story; pass null if the story isn't repost of another story
/// * `is_posted_to_chat_page` - Pass true to keep the story accessible after expiration
/// * `protect_content` - Pass true if the content of the story must be protected from forwarding and screenshotting
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Story)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn post_story(chat_id: i64, content: crate::enums::InputStoryContent, areas: Option<crate::types::InputStoryAreas>, caption: Option<crate::types::FormattedText>, privacy_settings: crate::enums::StoryPrivacySettings, album_ids: Vec<i32>, active_period: i32, from_story_full_id: Option<crate::types::StoryFullId>, is_posted_to_chat_page: bool, protect_content: bool, client_id: i32) -> Result<crate::enums::Story, crate::types::Error> {
    let request = json!({
        "@type": "postStory",
        "chat_id": chat_id,
        "content": content,
        "areas": areas,
        "caption": caption,
        "privacy_settings": privacy_settings,
        "album_ids": album_ids,
        "active_period": active_period,
        "from_story_full_id": from_story_full_id,
        "is_posted_to_chat_page": is_posted_to_chat_page,
        "protect_content": protect_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Starts a new live story on behalf of a chat; requires can_post_stories administrator right for channel chats
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that will start the live story. Pass Saved Messages chat identifier when starting a live story on behalf of the current user, or a channel chat identifier
/// * `privacy_settings` - The privacy settings for the story; ignored for stories posted on behalf of channel chats
/// * `protect_content` - Pass true if the content of the story must be protected from screenshotting
/// * `is_rtmp_stream` - Pass true to create an RTMP stream instead of an ordinary group call
/// * `enable_messages` - Pass true to allow viewers of the story to send messages
/// * `paid_message_star_count` - The minimum number of Telegram Stars that must be paid by viewers for each sent message to the call; 0-getOption("paid_group_call_message_star_count_max")
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StartLiveStoryResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn start_live_story(chat_id: i64, privacy_settings: crate::enums::StoryPrivacySettings, protect_content: bool, is_rtmp_stream: bool, enable_messages: bool, paid_message_star_count: i64, client_id: i32) -> Result<crate::enums::StartLiveStoryResult, crate::types::Error> {
    let request = json!({
        "@type": "startLiveStory",
        "chat_id": chat_id,
        "privacy_settings": privacy_settings,
        "protect_content": protect_content,
        "is_rtmp_stream": is_rtmp_stream,
        "enable_messages": enable_messages,
        "paid_message_star_count": paid_message_star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes content and caption of a story. Can be called only if story.can_be_edited == true
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to edit
/// * `content` - New content of the story; pass null to keep the current content
/// * `areas` - New clickable rectangle areas to be shown on the story media; pass null to keep the current areas. Areas can't be edited if story content isn't changed
/// * `caption` - New story caption; pass null to keep the current caption
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_story(story_poster_chat_id: i64, story_id: i32, content: Option<crate::enums::InputStoryContent>, areas: Option<crate::types::InputStoryAreas>, caption: Option<crate::types::FormattedText>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "content": content,
        "areas": areas,
        "caption": caption,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes cover of a video story. Can be called only if story.can_be_edited == true and the story isn't being edited now
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to edit
/// * `cover_frame_timestamp` - New timestamp of the frame, which will be used as video thumbnail
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_story_cover(story_poster_chat_id: i64, story_id: i32, cover_frame_timestamp: f64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editStoryCover",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "cover_frame_timestamp": cover_frame_timestamp,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes privacy settings of a story. The method can be called only for stories posted on behalf of the current user and if story.can_set_privacy_settings == true
///
/// # Arguments
///
/// * `story_id` - Identifier of the story
/// * `privacy_settings` - The new privacy settings for the story
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_story_privacy_settings(story_id: i32, privacy_settings: crate::enums::StoryPrivacySettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStoryPrivacySettings",
        "story_id": story_id,
        "privacy_settings": privacy_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether a story is accessible after expiration. Can be called only if story.can_toggle_is_posted_to_chat_page == true
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story
/// * `is_posted_to_chat_page` - Pass true to make the story accessible after expiration; pass false to make it private
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_story_is_posted_to_chat_page(story_poster_chat_id: i64, story_id: i32, is_posted_to_chat_page: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleStoryIsPostedToChatPage",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "is_posted_to_chat_page": is_posted_to_chat_page,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes a previously posted story. Can be called only if story.can_be_deleted == true
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_story(story_poster_chat_id: i64, story_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of chats with non-default notification settings for stories
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_story_notification_settings_exceptions(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getStoryNotificationSettingsExceptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Loads more active stories from a story list. The loaded stories will be sent through updates. Active stories are sorted by
/// the pair (active_stories.order, active_stories.story_poster_chat_id) in descending order. Returns a 404 error if all active stories have been loaded
///
/// # Arguments
///
/// * `story_list` - The story list in which to load active stories
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_active_stories(story_list: crate::enums::StoryList, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadActiveStories",
        "story_list": story_list,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes story list in which stories from the chat are shown
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that posted stories
/// * `story_list` - New list for active stories posted by the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_active_stories_list(chat_id: i64, story_list: crate::enums::StoryList, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatActiveStoriesList",
        "chat_id": chat_id,
        "story_list": story_list,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of active stories posted by the given chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatActiveStories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_active_stories(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatActiveStories, crate::types::Error> {
    let request = json!({
        "@type": "getChatActiveStories",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of stories that posted by the given chat to its chat page. If from_story_id == 0, then pinned stories are returned first.
/// Then, stories are returned in reverse chronological order (i.e., in order of decreasing story_id). For optimal performance, the number of returned stories is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `from_story_id` - Identifier of the story starting from which stories must be returned; use 0 to get results from pinned and the newest story
/// * `limit` - The maximum number of stories to be returned.
/// For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_posted_to_chat_page_stories(chat_id: i64, from_story_id: i32, limit: i32, client_id: i32) -> Result<crate::enums::Stories, crate::types::Error> {
    let request = json!({
        "@type": "getChatPostedToChatPageStories",
        "chat_id": chat_id,
        "from_story_id": from_story_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of all stories posted by the given chat; requires can_edit_stories administrator right in the chat.
/// The stories are returned in reverse chronological order (i.e., in order of decreasing story_id). For optimal performance, the number of returned stories is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `from_story_id` - Identifier of the story starting from which stories must be returned; use 0 to get results from the last story
/// * `limit` - The maximum number of stories to be returned.
/// For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_archived_stories(chat_id: i64, from_story_id: i32, limit: i32, client_id: i32) -> Result<crate::enums::Stories, crate::types::Error> {
    let request = json!({
        "@type": "getChatArchivedStories",
        "chat_id": chat_id,
        "from_story_id": from_story_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the list of pinned stories on a chat page; requires can_edit_stories administrator right in the chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that posted the stories
/// * `story_ids` - New list of pinned stories. All stories must be posted to the chat page first. There can be up to getOption("pinned_story_count_max") pinned stories on a chat page
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_pinned_stories(chat_id: i64, story_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatPinnedStories",
        "chat_id": chat_id,
        "story_ids": story_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that a story is opened and is being viewed by the user
///
/// # Arguments
///
/// * `story_poster_chat_id` - The identifier of the chat that posted the opened story
/// * `story_id` - The identifier of the story
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn open_story(story_poster_chat_id: i64, story_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that a story is closed by the user
///
/// # Arguments
///
/// * `story_poster_chat_id` - The identifier of the poster of the story to close
/// * `story_id` - The identifier of the story
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn close_story(story_poster_chat_id: i64, story_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns reactions, which can be chosen for a story
///
/// # Arguments
///
/// * `row_size` - Number of reaction per row, 5-25
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AvailableReactions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_story_available_reactions(row_size: i32, client_id: i32) -> Result<crate::enums::AvailableReactions, crate::types::Error> {
    let request = json!({
        "@type": "getStoryAvailableReactions",
        "row_size": row_size,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes chosen reaction on a story that has already been sent; not supported for live stories
///
/// # Arguments
///
/// * `story_poster_chat_id` - The identifier of the poster of the story
/// * `story_id` - The identifier of the story
/// * `reaction_type` - Type of the reaction to set; pass null to remove the reaction. Custom emoji reactions can be used only by Telegram Premium users. Paid reactions can't be set
/// * `update_recent_reactions` - Pass true if the reaction needs to be added to recent reactions
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_story_reaction(story_poster_chat_id: i64, story_id: i32, reaction_type: Option<crate::enums::ReactionType>, update_recent_reactions: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStoryReaction",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "reaction_type": reaction_type,
        "update_recent_reactions": update_recent_reactions,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns interactions with a story. The method can be called only for stories posted on behalf of the current user
///
/// # Arguments
///
/// * `story_id` - Story identifier
/// * `query` - Query to search for in names, usernames and titles; may be empty to get all relevant interactions
/// * `only_contacts` - Pass true to get only interactions by contacts; pass false to get all relevant interactions
/// * `prefer_forwards` - Pass true to get forwards and reposts first, then reactions, then other views; pass false to get interactions sorted just by interaction date
/// * `prefer_with_reaction` - Pass true to get interactions with reaction first; pass false to get interactions sorted just by interaction date. Ignored if prefer_forwards == true
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of story interactions to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryInteractions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_story_interactions(story_id: i32, query: String, only_contacts: bool, prefer_forwards: bool, prefer_with_reaction: bool, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::StoryInteractions, crate::types::Error> {
    let request = json!({
        "@type": "getStoryInteractions",
        "story_id": story_id,
        "query": query,
        "only_contacts": only_contacts,
        "prefer_forwards": prefer_forwards,
        "prefer_with_reaction": prefer_with_reaction,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns interactions with a story posted in a chat. Can be used only if story is posted on behalf of a chat and the user is an administrator in the chat
///
/// # Arguments
///
/// * `story_poster_chat_id` - The identifier of the poster of the story
/// * `story_id` - Story identifier
/// * `reaction_type` - Pass the default heart reaction or a suggested reaction type to receive only interactions with the specified reaction type; pass null to receive all interactions; reactionTypePaid isn't supported
/// * `prefer_forwards` - Pass true to get forwards and reposts first, then reactions, then other views; pass false to get interactions sorted just by interaction date
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of story interactions to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryInteractions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_story_interactions(story_poster_chat_id: i64, story_id: i32, reaction_type: Option<crate::enums::ReactionType>, prefer_forwards: bool, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::StoryInteractions, crate::types::Error> {
    let request = json!({
        "@type": "getChatStoryInteractions",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "reaction_type": reaction_type,
        "prefer_forwards": prefer_forwards,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Reports a story to the Telegram moderators
///
/// # Arguments
///
/// * `story_poster_chat_id` - The identifier of the poster of the story to report
/// * `story_id` - The identifier of the story to report
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `text` - Additional report details; 0-1024 characters; leave empty for the initial request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReportStoryResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_story(story_poster_chat_id: i64, story_id: i32, option_id: String, text: String, client_id: i32) -> Result<crate::enums::ReportStoryResult, crate::types::Error> {
    let request = json!({
        "@type": "reportStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "option_id": option_id,
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Activates stealth mode for stories, which hides all views of stories from the current user in the last "story_stealth_mode_past_period" seconds
/// and for the next "story_stealth_mode_future_period" seconds; for Telegram Premium users only
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn activate_story_stealth_mode(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "activateStoryStealthMode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns forwards of a story as a message to public chats and reposts by public channels. Can be used only if the story is posted on behalf of the current user or story.can_get_statistics == true.
/// For optimal performance, the number of returned messages and stories is chosen by TDLib
///
/// # Arguments
///
/// * `story_poster_chat_id` - The identifier of the poster of the story
/// * `story_id` - The identifier of the story
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages and stories to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PublicForwards)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_story_public_forwards(story_poster_chat_id: i64, story_id: i32, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::PublicForwards, crate::types::Error> {
    let request = json!({
        "@type": "getStoryPublicForwards",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of story albums owned by the given chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryAlbums)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_story_albums(chat_id: i64, client_id: i32) -> Result<crate::enums::StoryAlbums, crate::types::Error> {
    let request = json!({
        "@type": "getChatStoryAlbums",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of stories added to the given story album. For optimal performance, the number of returned stories is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `story_album_id` - Story album identifier
/// * `offset` - Offset of the first entry to return; use 0 to get results from the first album story
/// * `limit` - The maximum number of stories to be returned. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Stories)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_story_album_stories(chat_id: i64, story_album_id: i32, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Stories, crate::types::Error> {
    let request = json!({
        "@type": "getStoryAlbumStories",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates an album of stories; requires can_edit_stories administrator right for supergroup and channel chats
///
/// # Arguments
///
/// * `story_poster_chat_id` - Identifier of the chat that posted the stories
/// * `name` - Name of the album; 1-12 characters
/// * `story_ids` - Identifiers of stories to add to the album; 0-getOption("story_album_size_max") identifiers
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryAlbum)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_story_album(story_poster_chat_id: i64, name: String, story_ids: Vec<i32>, client_id: i32) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
        "@type": "createStoryAlbum",
        "story_poster_chat_id": story_poster_chat_id,
        "name": name,
        "story_ids": story_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes order of story albums. If the albums are owned by a supergroup or a channel chat, then requires can_edit_stories administrator right in the chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_ids` - New order of story albums
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_story_albums(chat_id: i64, story_album_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderStoryAlbums",
        "chat_id": chat_id,
        "story_album_ids": story_album_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes a story album. If the album is owned by a supergroup or a channel chat, then requires can_edit_stories administrator right in the chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_story_album(chat_id: i64, story_album_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteStoryAlbum",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes name of an album of stories. If the album is owned by a supergroup or a channel chat, then requires can_edit_stories administrator right in the chat. Returns the changed album
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `name` - New name of the album; 1-12 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryAlbum)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_story_album_name(chat_id: i64, story_album_id: i32, name: String, client_id: i32) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
        "@type": "setStoryAlbumName",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds stories to the beginning of a previously created story album. If the album is owned by a supergroup or a channel chat, then
/// requires can_edit_stories administrator right in the chat. Returns the changed album
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `story_ids` - Identifier of the stories to add to the album; 1-getOption("story_album_size_max") identifiers.
/// If after addition the album has more than getOption("story_album_size_max") stories, then the last one are removed from the album
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryAlbum)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_story_album_stories(chat_id: i64, story_album_id: i32, story_ids: Vec<i32>, client_id: i32) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
        "@type": "addStoryAlbumStories",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
        "story_ids": story_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes stories from an album. If the album is owned by a supergroup or a channel chat, then
/// requires can_edit_stories administrator right in the chat. Returns the changed album
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `story_ids` - Identifier of the stories to remove from the album
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryAlbum)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_story_album_stories(chat_id: i64, story_album_id: i32, story_ids: Vec<i32>, client_id: i32) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
        "@type": "removeStoryAlbumStories",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
        "story_ids": story_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes order of stories in an album. If the album is owned by a supergroup or a channel chat, then
/// requires can_edit_stories administrator right in the chat. Returns the changed album
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `story_ids` - Identifier of the stories to move to the beginning of the album. All other stories are placed in the current order after the specified stories
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryAlbum)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_story_album_stories(chat_id: i64, story_album_id: i32, story_ids: Vec<i32>, client_id: i32) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
        "@type": "reorderStoryAlbumStories",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
        "story_ids": story_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns RTMP URL for streaming to a live story; requires can_post_stories administrator right for channel chats
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RtmpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_rtmp_url(chat_id: i64, client_id: i32) -> Result<crate::enums::RtmpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getLiveStoryRtmpUrl",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Replaces the current RTMP URL for streaming to a live story; requires owner privileges for channel chats
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RtmpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn replace_live_story_rtmp_url(chat_id: i64, client_id: i32) -> Result<crate::enums::RtmpUrl, crate::types::Error> {
    let request = json!({
        "@type": "replaceLiveStoryRtmpUrl",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Joins a group call of an active live story. Returns join response payload for tgcalls
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `join_parameters` - Parameters to join the call
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn join_live_story(group_call_id: i32, join_parameters: crate::types::GroupCallJoinParameters, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "joinLiveStory",
        "group_call_id": group_call_id,
        "join_parameters": join_parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about the user or the chat that streams to a live story; for live stories that aren't an RTMP stream only
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GroupCallParticipant)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_streamer(group_call_id: i32, client_id: i32) -> Result<crate::enums::GroupCallParticipant, crate::types::Error> {
    let request = json!({
        "@type": "getLiveStoryStreamer",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of message sender identifiers, on whose behalf messages can be sent to a live story
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatMessageSenders)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_available_message_senders(group_call_id: i32, client_id: i32) -> Result<crate::enums::ChatMessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getLiveStoryAvailableMessageSenders",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Selects a message sender to send messages in a live story call
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `message_sender_id` - New message sender for the group call
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_live_story_message_sender(group_call_id: i32, message_sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setLiveStoryMessageSender",
        "group_call_id": group_call_id,
        "message_sender_id": message_sender_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds pending paid reaction in a live story group call. Can't be used in live stories posted by the current user.
/// Call commitPendingLiveStoryReactions or removePendingLiveStoryReactions to actually send all pending reactions when the undo timer is over or abort the sending
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `star_count` - Number of Telegram Stars to be used for the reaction. The total number of pending paid reactions must not exceed getOption("paid_group_call_message_star_count_max")
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_pending_live_story_reaction(group_call_id: i32, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addPendingLiveStoryReaction",
        "group_call_id": group_call_id,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Applies all pending paid reactions in a live story group call
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn commit_pending_live_story_reactions(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "commitPendingLiveStoryReactions",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes all pending paid reactions in a live story group call
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_pending_live_story_reactions(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removePendingLiveStoryReactions",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of top live story donors
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier of the live story
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LiveStoryDonors)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_top_donors(group_call_id: i32, client_id: i32) -> Result<crate::enums::LiveStoryDonors, crate::types::Error> {
    let request = json!({
        "@type": "getLiveStoryTopDonors",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns messages in the personal chat of a given user; for bots only
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `limit` - The maximum number of messages to be returned; 1-20
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_personal_chat_history(user_id: i64, limit: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getPersonalChatHistory",
        "user_id": user_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Toggles whether the message history of a supergroup is available to new members; requires can_change_info member right
///
/// # Arguments
///
/// * `supergroup_id` - The identifier of the supergroup
/// * `is_all_history_available` - The new value of is_all_history_available
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_is_all_history_available(supergroup_id: i64, is_all_history_available: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupIsAllHistoryAvailable",
        "supergroup_id": supergroup_id,
        "is_all_history_available": is_all_history_available,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns detailed statistics about a story. Can be used only if story.can_get_statistics == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `story_id` - Story identifier
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StoryStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_story_statistics(chat_id: i64, story_id: i32, is_dark: bool, client_id: i32) -> Result<crate::enums::StoryStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getStoryStatistics",
        "chat_id": chat_id,
        "story_id": story_id,
        "is_dark": is_dark,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

