//!
//! TDLib `message` domain functions.
//!
//! Types, enums, and functions for message content, rich formatting, reactions, and drafts.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns information about a message. Returns a 404 error if the message doesn't exist
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `message_id` - Identifier of the message to get
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getMessage",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a non-bundled message that is replied by a given message. Also, returns the pinned message for messagePinMessage,
/// the game message for messageGameScore, the invoice message for messagePaymentSuccessful, the message with a previously set same background for messageChatSetBackground,
/// the giveaway message for messageGiveawayCompleted, the checklist message for messageChecklistTasksDone, messageChecklistTasksAdded, the message with suggested post information
/// for messageSuggestedPostApprovalFailed, messageSuggestedPostApproved, messageSuggestedPostDeclined, messageSuggestedPostPaid, messageSuggestedPostRefunded,
/// the message with the regular gift that was upgraded for messageUpgradedGift with origin of the type upgradedGiftOriginUpgrade,
/// the message with gift purchase offer for messageUpgradedGiftPurchaseOfferRejected,
/// the message with the request to disable content protection for messageChatHasProtectedContentToggled,
/// the message with the poll for messagePollOptionAdded and messagePollOptionDeleted,
/// and the topic creation message for topic messages without non-bundled replied message. Returns a 404 error if the message doesn't exist
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `message_id` - Identifier of the reply message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_replied_message(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getRepliedMessage",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a newest pinned message in the chat. Returns a 404 error if the message doesn't exist
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_pinned_message(chat_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getChatPinnedMessage",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about messages. If a message is not found, returns null on the corresponding position of the result
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat the messages belong to
/// * `message_ids` - Identifiers of the messages to get
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_messages(chat_id: i64, message_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getMessages",
        "chat_id": chat_id,
        "message_ids": message_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the full version of a rich message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat the messages belong to
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RichMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_full_rich_message(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::RichMessage, crate::types::Error> {
    let request = json!({
        "@type": "getFullRichMessage",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns properties of a message. This is an offline method
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageProperties)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_properties(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::MessageProperties, crate::types::Error> {
    let request = json!({
        "@type": "getMessageProperties",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a message thread. Can be used only if messageProperties.can_get_message_thread == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageThreadInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_thread(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::MessageThreadInfo, crate::types::Error> {
    let request = json!({
        "@type": "getMessageThread",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns read date of a recent outgoing message in a private chat. The method can be called if messageProperties.can_get_read_date == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageReadDate)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_read_date(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::MessageReadDate, crate::types::Error> {
    let request = json!({
        "@type": "getMessageReadDate",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns viewers of a recent outgoing message in a basic group or a supergroup chat. For video notes and voice notes only users, opened content of the message, are returned. The method can be called if messageProperties.can_get_viewers == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageViewers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_viewers(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::MessageViewers, crate::types::Error> {
    let request = json!({
        "@type": "getMessageViewers",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about actual author of a message sent on behalf of a channel. The method can be called if messageProperties.can_get_author == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_author(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getMessageAuthor",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query
/// (searchSecretMessages must be used instead), or without an enabled message database. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit.
/// A combination of query, sender_id, filter and topic_id search criteria is expected to be supported, only if it is required for Telegram official application implementation
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which to search messages
/// * `topic_id` - Pass topic identifier to search messages only in specific topic; pass null to search for messages in all topics
/// * `query` - Query to search for
/// * `sender_id` - Identifier of the sender of messages to search for; pass null to search for messages from any sender. Not supported in secret chats
/// * `from_message_id` - Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number to get the specified message and some newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `filter` - Additional filter for messages to search; pass null to search for all messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundChatMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_messages(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, query: String, sender_id: Option<crate::enums::MessageSender>, from_message_id: i64, offset: i32, limit: i32, filter: Option<crate::enums::SearchMessagesFilter>, client_id: i32) -> Result<crate::enums::FoundChatMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchChatMessages",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "query": query,
        "sender_id": sender_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
        "filter": filter,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)).
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
///
/// # Arguments
///
/// * `chat_list` - Chat list in which to search messages; pass null to search in all chats regardless of their chat list. Only Main and Archive chat lists are supported
/// * `query` - Query to search for
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `filter` - Additional filter for messages to search; pass null to search for all messages. Filters searchMessagesFilterMention, searchMessagesFilterUnreadMention, searchMessagesFilterUnreadReaction,
/// searchMessagesFilterUnreadPollVote, searchMessagesFilterFailedToSend, and searchMessagesFilterPinned are unsupported in this function
/// * `chat_type_filter` - Additional filter for type of the chat of the searched messages; pass null to search for messages in all chats
/// * `min_date` - If not 0, the minimum date of the messages to return
/// * `max_date` - If not 0, the maximum date of the messages to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_messages(chat_list: Option<crate::enums::ChatList>, query: String, offset: String, limit: i32, filter: Option<crate::enums::SearchMessagesFilter>, chat_type_filter: Option<crate::enums::SearchMessagesChatTypeFilter>, min_date: i32, max_date: i32, client_id: i32) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchMessages",
        "chat_list": chat_list,
        "query": query,
        "offset": offset,
        "limit": limit,
        "filter": filter,
        "chat_type_filter": chat_type_filter,
        "min_date": min_date,
        "max_date": max_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance, the number of returned messages is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which to search. Specify 0 to search in all secret chats
/// * `query` - Query to search for. If empty, searchChatMessages must be used instead
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `filter` - Additional filter for messages to search; pass null to search for all messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_secret_messages(chat_id: i64, query: String, offset: String, limit: i32, filter: Option<crate::enums::SearchMessagesFilter>, client_id: i32) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchSecretMessages",
        "chat_id": chat_id,
        "query": query,
        "offset": offset,
        "limit": limit,
        "filter": filter,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for messages tagged by the given reaction and with the given words in the Saved Messages chat; for Telegram Premium users only.
/// Returns the results in reverse chronological order, i.e. in order of decreasing message_id.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
///
/// # Arguments
///
/// * `saved_messages_topic_id` - If not 0, only messages in the specified Saved Messages topic will be considered; pass 0 to consider all messages
/// * `tag` - Tag to search for; pass null to return all suitable messages
/// * `query` - Query to search for
/// * `from_message_id` - Identifier of the message starting from which messages must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number to get the specified message and some newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundChatMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_saved_messages(saved_messages_topic_id: i64, tag: Option<crate::enums::ReactionType>, query: String, from_message_id: i64, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::FoundChatMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchSavedMessages",
        "saved_messages_topic_id": saved_messages_topic_id,
        "tag": tag,
        "query": query,
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

/// Searches for public channel posts containing the given hashtag or cashtag. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
///
/// # Arguments
///
/// * `tag` - Hashtag or cashtag to search for
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_public_messages_by_tag(tag: String, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicMessagesByTag",
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

/// Returns information about the recent live locations of chat members that were sent to the chat. Returns at most one live location message per user
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `limit` - The maximum number of messages to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_chat_recent_location_messages(chat_id: i64, limit: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "searchChatRecentLocationMessages",
        "chat_id": chat_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the last message sent in a chat no later than the specified date. Returns a 404 error if such message doesn't exist
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `date` - Point in time (Unix timestamp) relative to which to search for messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_by_date(chat_id: i64, date: i32, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getChatMessageByDate",
        "chat_id": chat_id,
        "date": date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns sparse positions of messages of the specified type in the chat to be used for Shared Media scroll implementation. Returns the results in reverse chronological order (i.e., in order of decreasing message_id).
/// Cannot be used in secret chats or with searchMessagesFilterFailedToSend filter without an enabled message database
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which to return information about message positions
/// * `filter` - Filter for message content. Filters searchMessagesFilterEmpty, searchMessagesFilterMention, searchMessagesFilterUnreadMention, searchMessagesFilterUnreadReaction,
/// and searchMessagesFilterUnreadPollVote are unsupported in this function
/// * `from_message_id` - The message identifier from which to return information about message positions
/// * `limit` - The expected number of message positions to be returned; 50-2000. A smaller number of positions can be returned, if there are not enough appropriate messages
/// * `saved_messages_topic_id` - If not 0, only messages in the specified Saved Messages topic will be considered; pass 0 to consider all messages, or for chats other than Saved Messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessagePositions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_sparse_message_positions(chat_id: i64, filter: crate::enums::SearchMessagesFilter, from_message_id: i64, limit: i32, saved_messages_topic_id: i64, client_id: i32) -> Result<crate::enums::MessagePositions, crate::types::Error> {
    let request = json!({
        "@type": "getChatSparseMessagePositions",
        "chat_id": chat_id,
        "filter": filter,
        "from_message_id": from_message_id,
        "limit": limit,
        "saved_messages_topic_id": saved_messages_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about the next messages of the specified type in the chat split by days. Returns the results in reverse chronological order. Can return partial result for the last returned day. Behavior of this method depends on the value of the option "utc_time_offset"
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which to return information about messages
/// * `topic_id` - Pass topic identifier to get the result only in specific topic; pass null to get the result in all topics; forum topics and message threads aren't supported
/// * `filter` - Filter for message content. Filters searchMessagesFilterEmpty, searchMessagesFilterMention, searchMessagesFilterUnreadMention, searchMessagesFilterUnreadReaction,
/// and searchMessagesFilterUnreadPollVote are unsupported in this function
/// * `from_message_id` - The message identifier from which to return information about messages; use 0 to get results from the last message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageCalendar)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_calendar(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, filter: crate::enums::SearchMessagesFilter, from_message_id: i64, client_id: i32) -> Result<crate::enums::MessageCalendar, crate::types::Error> {
    let request = json!({
        "@type": "getChatMessageCalendar",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "filter": filter,
        "from_message_id": from_message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns approximate number of messages of the specified type in the chat or its topic
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which to count messages
/// * `topic_id` - Pass topic identifier to get number of messages only in specific topic; pass null to get number of messages in all topics; message threads aren't supported
/// * `filter` - Filter for message content; searchMessagesFilterEmpty is unsupported in this function
/// * `return_local` - Pass true to get the number of messages without sending network requests, or -1 if the number of messages is unknown locally
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Count)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_count(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, filter: crate::enums::SearchMessagesFilter, return_local: bool, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getChatMessageCount",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "filter": filter,
        "return_local": return_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns approximate 1-based position of a message among messages, which can be found by the specified filter in the chat and topic. Cannot be used in secret chats
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which to find message position
/// * `topic_id` - Pass topic identifier to get position among messages only in specific topic; pass null to get position among all chat messages; message threads aren't supported
/// * `filter` - Filter for message content; searchMessagesFilterEmpty, searchMessagesFilterUnreadMention, searchMessagesFilterUnreadReaction, searchMessagesFilterUnreadPollVote,
/// and searchMessagesFilterFailedToSend are unsupported in this function
/// * `message_id` - Message identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Count)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_position(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, filter: crate::enums::SearchMessagesFilter, message_id: i64, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getChatMessagePosition",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "filter": filter,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns all scheduled messages in a chat. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id)
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_scheduled_messages(chat_id: i64, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getChatScheduledMessages",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns sponsored messages to be shown in a chat; for channel chats and chats with bots only
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::SponsoredMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_sponsored_messages(chat_id: i64, client_id: i32) -> Result<crate::enums::SponsoredMessages, crate::types::Error> {
    let request = json!({
        "@type": "getChatSponsoredMessages",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that the user opened the sponsored chat via the button, the name, the chat photo, a mention in the sponsored message text, or the media in the sponsored message
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the sponsored message
/// * `message_id` - Identifier of the sponsored message
/// * `is_media_click` - Pass true if the media was clicked in the sponsored message
/// * `from_fullscreen` - Pass true if the user expanded the video from the sponsored message fullscreen before the click
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn click_chat_sponsored_message(chat_id: i64, message_id: i64, is_media_click: bool, from_fullscreen: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clickChatSponsoredMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "is_media_click": is_media_click,
        "from_fullscreen": from_fullscreen,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports a sponsored message to Telegram moderators
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the sponsored message
/// * `message_id` - Identifier of the sponsored message
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReportSponsoredResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_chat_sponsored_message(chat_id: i64, message_id: i64, option_id: String, client_id: i32) -> Result<crate::enums::ReportSponsoredResult, crate::types::Error> {
    let request = json!({
        "@type": "reportChatSponsoredMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "option_id": option_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTPS link to a message in a chat. Available only if messageProperties.can_get_link, or if messageProperties.can_get_media_timestamp_links and a media timestamp link is generated. This is an offline method
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `media_timestamp` - If not 0, timestamp from which the video/audio/video note/voice note/story playing must start, in seconds. The media can be in the message content or in its link preview
/// * `checklist_task_id` - If not 0, identifier of the checklist task in the message to be linked
/// * `poll_option_id` - If not empty, identifier of the poll option in the message to be linked
/// * `for_album` - Pass true to create a link for the whole media album
/// * `in_message_thread` - Pass true to create a link to the message as a channel post comment, in a message thread, or a forum topic
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageLink)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_link(chat_id: i64, message_id: i64, media_timestamp: i32, checklist_task_id: i32, poll_option_id: String, for_album: bool, in_message_thread: bool, client_id: i32) -> Result<crate::enums::MessageLink, crate::types::Error> {
    let request = json!({
        "@type": "getMessageLink",
        "chat_id": chat_id,
        "message_id": message_id,
        "media_timestamp": media_timestamp,
        "checklist_task_id": checklist_task_id,
        "poll_option_id": poll_option_id,
        "for_album": for_album,
        "in_message_thread": in_message_thread,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTML code for embedding the message. Available only if messageProperties.can_get_embedding_code
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `for_album` - Pass true to return an HTML code for embedding of the whole media album
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_embedding_code(chat_id: i64, message_id: i64, for_album: bool, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getMessageEmbeddingCode",
        "chat_id": chat_id,
        "message_id": message_id,
        "for_album": for_album,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a public or private message link. Can be called for any internal link of the type internalLinkTypeMessage
///
/// # Arguments
///
/// * `url` - The message link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageLinkInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_link_info(url: String, client_id: i32) -> Result<crate::enums::MessageLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "getMessageLinkInfo",
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a custom text composition style. May return an error with a message "TONES_SAVED_TOO_MANY" if the maximum number of added custom styles has been reached
///
/// # Arguments
///
/// * `title` - Title of the style; 1-getOption("text_composition_style_title_length_max") characters
/// * `custom_emoji_id` - Identifier of the custom emoji corresponding to the style
/// * `prompt` - Prompt that will be used for text composition; 1-getOption("text_composition_style_prompt_length_max") characters
/// * `show_creator` - Pass true if the current user must be shown as the creator of the style
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TextCompositionStyle)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_text_composition_style(title: String, custom_emoji_id: i64, prompt: String, show_creator: bool, client_id: i32) -> Result<crate::enums::TextCompositionStyle, crate::types::Error> {
    let request = json!({
        "@type": "createTextCompositionStyle",
        "title": title,
        "custom_emoji_id": custom_emoji_id,
        "prompt": prompt,
        "show_creator": show_creator,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits a custom text composition style that was created by the current user
///
/// # Arguments
///
/// * `name` - Name of the style
/// * `title` - Title of the style; 1-getOption("text_composition_style_title_length_max") characters
/// * `custom_emoji_id` - Identifier of the custom emoji corresponding to the style
/// * `prompt` - Prompt that will be used for text composition; 1-getOption("text_composition_style_prompt_length_max") characters
/// * `show_creator` - Pass true if the current user must be shown as the creator of the style
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TextCompositionStyle)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_text_composition_style(name: String, title: String, custom_emoji_id: i64, prompt: String, show_creator: bool, client_id: i32) -> Result<crate::enums::TextCompositionStyle, crate::types::Error> {
    let request = json!({
        "@type": "editTextCompositionStyle",
        "name": name,
        "title": title,
        "custom_emoji_id": custom_emoji_id,
        "prompt": prompt,
        "show_creator": show_creator,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes a custom text composition style that was created by the current user
///
/// # Arguments
///
/// * `name` - Name of the style
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_text_composition_style(name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteTextCompositionStyle",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches a custom text composition style by its name
///
/// # Arguments
///
/// * `name` - Name of the style
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TextCompositionStyle)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_text_composition_style(name: String, client_id: i32) -> Result<crate::enums::TextCompositionStyle, crate::types::Error> {
    let request = json!({
        "@type": "searchTextCompositionStyle",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an example of usage of a custom text composition style
///
/// # Arguments
///
/// * `name` - Name of the style
/// * `example_number` - 0-based unique number of the requested example; must be non-negative and less than getOption("text_composition_style_example_count")
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TextCompositionStyleExample)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_text_composition_style_example(name: String, example_number: i32, client_id: i32) -> Result<crate::enums::TextCompositionStyleExample, crate::types::Error> {
    let request = json!({
        "@type": "getTextCompositionStyleExample",
        "name": name,
        "example_number": example_number,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a custom text composition style to the list of used by the user styles. May return an error with a message "TONES_SAVED_TOO_MANY"
/// if the maximum number of added custom styles getOption("added_text_composition_style_count_max") has been reached
///
/// # Arguments
///
/// * `name` - Name of the style
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_text_composition_style(name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addTextCompositionStyle",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a custom text composition style from the list of used by the user styles. If the style was created by the current user, then it can only be deleted
///
/// # Arguments
///
/// * `name` - Name of the style
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_text_composition_style(name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeTextCompositionStyle",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Translates a text to the given language; must not be used in secret chats. If the current user is a Telegram Premium user, then text formatting is preserved
///
/// # Arguments
///
/// * `text` - Text to translate
/// * `to_language_code` - Language code of the language to which the message is translated. Must be one of
/// "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "zh-CN", "zh", "zh-Hans", "zh-TW", "zh-Hant", "co", "hr", "cs", "da", "nl", "en", "eo", "et",
/// "fi", "fr", "fy", "gl", "ka", "de", "el", "gu", "ht", "ha", "haw", "he", "iw", "hi", "hmn", "hu", "is", "ig", "id", "in", "ga", "it", "ja", "jv", "kn", "kk", "km", "rw", "ko",
/// "ku", "ky", "lo", "la", "lv", "lt", "lb", "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mn", "my", "ne", "no", "ny", "or", "ps", "fa", "pl", "pt", "pt-BR", "pa", "ro", "ru", "sm", "gd", "sr",
/// "st", "sn", "sd", "si", "sk", "sl", "so", "es", "su", "sw", "sv", "tl", "tg", "ta", "tt", "te", "th", "tr", "tk", "uk", "ur", "ug", "uz", "vi", "cy", "xh", "yi", "ji", "yo", "zu"
/// * `tone` - Tone of the translation; must be one of "", "formal", "neutral", "casual"; defaults to "neutral"
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FormattedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn translate_text(text: crate::types::FormattedText, to_language_code: String, tone: String, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "translateText",
        "text": text,
        "to_language_code": to_language_code,
        "tone": tone,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Translates a rich message to the given language
///
/// # Arguments
///
/// * `message` - Rich message to translate
/// * `to_language_code` - Language code of the language to which the message is translated. See translateText.to_language_code for the list of supported values
/// * `tone` - Tone of the translation; see translateText.tone for the list of supported values
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RichMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn translate_rich_message(message: crate::types::InputRichMessage, to_language_code: String, tone: String, client_id: i32) -> Result<crate::enums::RichMessage, crate::types::Error> {
    let request = json!({
        "@type": "translateRichMessage",
        "message": message,
        "to_language_code": to_language_code,
        "tone": tone,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Extracts text or caption of the given message and translates it to the given language; must not be used in secret chats. If the current user is a Telegram Premium user, then text formatting is preserved
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `to_language_code` - Language code of the language to which the message is translated. See translateText.to_language_code for the list of supported values
/// * `tone` - Tone of the translation; see translateText.tone for the list of supported values
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FormattedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn translate_message_text(chat_id: i64, message_id: i64, to_language_code: String, tone: String, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "translateMessageText",
        "chat_id": chat_id,
        "message_id": message_id,
        "to_language_code": to_language_code,
        "tone": tone,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Extracts rich message of the given message and translates it to the given language
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `to_language_code` - Language code of the language to which the message is translated. See translateText.to_language_code for the list of supported values
/// * `tone` - Tone of the translation; see translateText.tone for the list of supported values
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RichMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn translate_message_rich_message(chat_id: i64, message_id: i64, to_language_code: String, tone: String, client_id: i32) -> Result<crate::enums::RichMessage, crate::types::Error> {
    let request = json!({
        "@type": "translateMessageRichMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "to_language_code": to_language_code,
        "tone": tone,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Summarizes content of the message with non-empty summary_language_code
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `translate_to_language_code` - Pass a language code to which the summary will be translated; pass an empty string if translation isn't needed. See translateText.to_language_code for the list of supported values
/// * `tone` - Tone of the summarization; see translateText.tone for the list of supported values
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FormattedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn summarize_message(chat_id: i64, message_id: i64, translate_to_language_code: String, tone: String, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "summarizeMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "translate_to_language_code": translate_to_language_code,
        "tone": tone,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes text using an AI model; must not be used in secret chats. May return an error with a message "AICOMPOSE_FLOOD_PREMIUM" if Telegram Premium is required to send further requests
///
/// # Arguments
///
/// * `text` - The original text
/// * `translate_to_language_code` - Pass a language code to which the text will be translated; pass an empty string if translation isn't needed. See translateText.to_language_code for the list of supported values
/// * `style_name` - Name of the style of the resulted text; handle updateTextCompositionStyles to get the list of supported styles; pass an empty string to keep the current style of the text
/// * `add_emojis` - Pass true to add emoji to the text
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FormattedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn compose_text_with_ai(text: crate::types::FormattedText, translate_to_language_code: String, style_name: String, add_emojis: bool, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "composeTextWithAi",
        "text": text,
        "translate_to_language_code": translate_to_language_code,
        "style_name": style_name,
        "add_emojis": add_emojis,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes a rich message using an AI model. May return an error with a message "AICOMPOSE_FLOOD_PREMIUM" if Telegram Premium is required to send further requests
///
/// # Arguments
///
/// * `message` - The original message
/// * `translate_to_language_code` - Pass a language code to which the text will be translated; pass an empty string if translation isn't needed. See translateText.to_language_code for the list of supported values
/// * `style_name` - Name of the style of the resulted text; handle updateTextCompositionStyles to get the list of supported styles; pass an empty string to keep the current style of the text or if a custom prompt is used
/// * `custom_prompt` - Custom prompt that will be used instead of style_name; 0-getOption("text_composition_style_prompt_length_max") characters
/// * `add_emojis` - Pass true to add emoji to the text
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RichMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn compose_rich_message_with_ai(message: crate::types::InputRichMessage, translate_to_language_code: String, style_name: String, custom_prompt: String, add_emojis: bool, client_id: i32) -> Result<crate::enums::RichMessage, crate::types::Error> {
    let request = json!({
        "@type": "composeRichMessageWithAi",
        "message": message,
        "translate_to_language_code": translate_to_language_code,
        "style_name": style_name,
        "custom_prompt": custom_prompt,
        "add_emojis": add_emojis,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new rich message using an AI model. May return an error with a message "AICOMPOSE_FLOOD_PREMIUM" if Telegram Premium is required to send further requests
///
/// # Arguments
///
/// * `prompt` - Prompt that will be used to create the message; 0-getOption("text_composition_style_prompt_length_max") characters
/// * `language_code` - Pass a language code in which the text will be created
/// * `add_emojis` - Pass true to add emoji to the text
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RichMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_rich_message_with_ai(prompt: String, language_code: String, add_emojis: bool, client_id: i32) -> Result<crate::enums::RichMessage, crate::types::Error> {
    let request = json!({
        "@type": "createRichMessageWithAi",
        "prompt": prompt,
        "language_code": language_code,
        "add_emojis": add_emojis,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Fixes text using an AI model; must not be used in secret chats. May return an error with a message "AICOMPOSE_FLOOD_PREMIUM" if Telegram Premium is required to send further requests
///
/// # Arguments
///
/// * `text` - The original text
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FixedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn fix_text_with_ai(text: crate::types::FormattedText, client_id: i32) -> Result<crate::enums::FixedText, crate::types::Error> {
    let request = json!({
        "@type": "fixTextWithAi",
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Fixes a rich message using an AI model. May return an error with a message "AICOMPOSE_FLOOD_PREMIUM" if Telegram Premium is required to send further requests
///
/// # Arguments
///
/// * `message` - The original message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RichMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn fix_rich_message_with_ai(message: crate::types::InputRichMessage, client_id: i32) -> Result<crate::enums::RichMessage, crate::types::Error> {
    let request = json!({
        "@type": "fixRichMessageWithAi",
        "message": message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of message sender identifiers, which can be used to send messages in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ChatMessageSenders)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_available_message_senders(chat_id: i64, client_id: i32) -> Result<crate::enums::ChatMessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getChatAvailableMessageSenders",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Selects a message sender to send messages in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_sender_id` - New message sender for the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_message_sender(chat_id: i64, message_sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatMessageSender",
        "chat_id": chat_id,
        "message_sender_id": message_sender_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a message. Returns the sent message
///
/// # Arguments
///
/// * `chat_id` - Target chat
/// * `topic_id` - Topic in which the message will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `options` - Options to be used to send the message; pass null to use default options
/// * `reply_markup` - Markup for replying to the message; pass null if none; for bots only
/// * `input_message_content` - The content of the message to be sent
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_message(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, reply_to: Option<crate::enums::InputMessageReplyTo>, options: Option<crate::types::MessageSendOptions>, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "sendMessage",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "reply_to": reply_to,
        "options": options,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends 2-10 messages grouped together into an album. Currently, only audio, document, photo and video messages can be grouped into an album.
/// Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
///
/// # Arguments
///
/// * `chat_id` - Target chat
/// * `topic_id` - Topic in which the messages will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `options` - Options to be used to send the messages; pass null to use default options
/// * `input_message_contents` - Contents of messages to be sent. At most 10 messages can be added to an album. All messages must have the same value of show_caption_above_media
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_message_album(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, reply_to: Option<crate::enums::InputMessageReplyTo>, options: Option<crate::types::MessageSendOptions>, input_message_contents: Vec<crate::enums::InputMessageContent>, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "sendMessageAlbum",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "reply_to": reply_to,
        "options": options,
        "input_message_contents": input_message_contents,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which to forward messages
/// * `topic_id` - Topic in which the messages will be forwarded; message threads aren't supported; pass null if none
/// * `from_chat_id` - Identifier of the chat from which to forward messages
/// * `message_ids` - Identifiers of the messages to forward. Message identifiers must be in a strictly increasing order. At most 100 messages can be forwarded simultaneously. A message can be forwarded only if messageProperties.can_be_forwarded
/// * `options` - Options to be used to send the messages; pass null to use default options
/// * `send_copy` - Pass true to copy content of the messages without reference to the original sender. Always true if the messages are forwarded to a secret chat or are local.
/// Use messageProperties.can_be_copied and messageProperties.can_be_copied_to_secret_chat to check whether the message is suitable
/// * `remove_caption` - Pass true to remove media captions of message copies. Ignored if send_copy is false
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn forward_messages(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, from_chat_id: i64, message_ids: Vec<i64>, options: Option<crate::types::MessageSendOptions>, send_copy: bool, remove_caption: bool, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "forwardMessages",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "from_chat_id": from_chat_id,
        "message_ids": message_ids,
        "options": options,
        "send_copy": send_copy,
        "remove_caption": remove_caption,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends messages from a quick reply shortcut. Requires Telegram Business subscription. Can't be used to send paid messages
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which to send messages. The chat must be a private chat with a regular user
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `sending_id` - Non-persistent identifier, which will be returned back in messageSendingStatePending object and can be used to match sent messages and corresponding updateNewMessage updates
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_quick_reply_shortcut_messages(chat_id: i64, shortcut_id: i32, sending_id: i32, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "sendQuickReplyShortcutMessages",
        "chat_id": chat_id,
        "shortcut_id": shortcut_id,
        "sending_id": sending_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed.
/// If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to send messages
/// * `message_ids` - Identifiers of the messages to resend. Message identifiers must be in a strictly increasing order
/// * `quote` - New manually chosen quote from the message to be replied; pass null if none. Ignored if more than one message is re-sent, or if messageSendingStateFailed.need_another_reply_quote == false
/// * `paid_message_star_count` - The number of Telegram Stars the user agreed to pay to send the messages. Ignored if messageSendingStateFailed.required_paid_message_star_count == 0
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Messages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn resend_messages(chat_id: i64, message_ids: Vec<i64>, quote: Option<crate::types::InputTextQuote>, paid_message_star_count: i64, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "resendMessages",
        "chat_id": chat_id,
        "message_ids": message_ids,
        "quote": quote,
        "paid_message_star_count": paid_message_star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends an ephemeral message which will be received only by one bot in a chat. Currently, only ephemeral bot commands and replies to bot ephemeral messages can be sent using the method.
/// The message is persistent across application restarts only if the message database is used. Returns the sent message
///
/// # Arguments
///
/// * `chat_id` - Target chat
/// * `topic_id` - Topic in which the message will be sent; pass null if none
/// * `receiver_user_id` - Identifier of the user who will receive the message
/// * `callback_query_id` - Identifier of the callback query which triggered the message; for bots only
/// * `replace_callback_query_message` - Pass true if the ephemeral message must replace the message from which the callback query originated; for bots only
/// * `reply_to` - Information about the message to be replied; pass null if none. The message can be an incoming ephemeral message
/// * `protect_content` - Pass true if the content of the message must be protected from forwarding and saving; for bots only
/// * `sending_id` - Non-persistent identifier, which will be returned back in messageSendingStatePending object and can be used to match sent messages and corresponding updateNewMessage updates
/// * `only_preview` - Pass true to get a fake message instead of actually sending them
/// * `reply_markup` - Markup for replying to the message; pass null if none; for bots only
/// * `input_message_content` - The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageAnimation,
/// inputMessageAudio, inputMessageDocument, inputMessagePhoto, inputMessageRichMessage, inputMessageSticker, inputMessageVideo, inputMessageVideoNote, inputMessageVoiceNote,
/// inputMessageLocation, inputMessageVenue, inputMessageContact
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_ephemeral_message(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, receiver_user_id: i64, callback_query_id: i64, replace_callback_query_message: bool, reply_to: Option<crate::enums::InputMessageReplyTo>, protect_content: bool, sending_id: i32, only_preview: bool, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "sendEphemeralMessage",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "receiver_user_id": receiver_user_id,
        "callback_query_id": callback_query_id,
        "replace_callback_query_message": replace_callback_query_message,
        "reply_to": reply_to,
        "protect_content": protect_content,
        "sending_id": sending_id,
        "only_preview": only_preview,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message
///
/// # Arguments
///
/// * `chat_id` - Target chat; channel direct messages chats aren't supported
/// * `sender_id` - Identifier of the sender of the message
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `disable_notification` - Pass true to disable notification for the message
/// * `input_message_content` - The content of the message to be added
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_local_message(chat_id: i64, sender_id: crate::enums::MessageSender, reply_to: Option<crate::enums::InputMessageReplyTo>, disable_notification: bool, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "addLocalMessage",
        "chat_id": chat_id,
        "sender_id": sender_id,
        "reply_to": reply_to,
        "disable_notification": disable_notification,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes messages
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_ids` - Identifiers of the messages to be deleted. Use messageProperties.can_be_deleted_only_for_self and messageProperties.can_be_deleted_for_all_users to get suitable messages
/// * `revoke` - Pass true to delete messages for all chat members. Always true for supergroups, channels and secret chats
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_messages(chat_id: i64, message_ids: Vec<i64>, revoke: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteMessages",
        "chat_id": chat_id,
        "message_ids": message_ids,
        "revoke": revoke,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes an ephemeral message; for bots only
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `receiver_user_id` - Identifier of the user who received the message
/// * `ephemeral_message_id` - Identifier of the message to be deleted
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_ephemeral_message(chat_id: i64, receiver_user_id: i64, ephemeral_message_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteEphemeralMessage",
        "chat_id": chat_id,
        "receiver_user_id": receiver_user_id,
        "ephemeral_message_id": ephemeral_message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all messages sent by the specified message sender in a chat. Supported only for supergroups; requires can_delete_messages administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `sender_id` - Identifier of the sender of messages to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_messages_by_sender(chat_id: i64, sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatMessagesBySender",
        "chat_id": chat_id,
        "sender_id": sender_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all messages between the specified dates in a chat. Supported only for private chats and basic groups. Messages sent in the last 30 seconds will not be deleted
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `min_date` - The minimum date of the messages to delete
/// * `max_date` - The maximum date of the messages to delete
/// * `revoke` - Pass true to delete chat messages for all users; private chats only
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_messages_by_date(chat_id: i64, min_date: i32, max_date: i32, revoke: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatMessagesByDate",
        "chat_id": chat_id,
        "min_date": min_date,
        "max_date": max_date,
        "revoke": revoke,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `input_message_content` - New text content of the message. Must be of type inputMessageText or inputMessageRichMessage
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_text(chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageText",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location.
/// Returns the edited message after the edit is completed on the server side
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `location` - New live location of the message; pass null to stop sharing the live location. If the new live_period isn't set to 0x7FFFFFFF,
/// then it must not exceed the current live_period by more than a day, and the live location expiration date must remain in the next 90 days
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_live_location(chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, location: Option<crate::types::LiveLocation>, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageLiveLocation",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the message content of a checklist. Returns the edited message after the edit is completed on the server side
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `checklist` - The new checklist. If some tasks were completed, this information will be kept
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_checklist(chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, checklist: crate::types::InputChecklist, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageChecklist",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "checklist": checklist,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the media content of a message, including message caption. If only the caption needs to be edited, use editMessageCaption instead.
/// The type of message content in an album can't be changed with exception of replacing a photo with a video or vice versa. Returns the edited message after the edit is completed on the server side
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_edit_media to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessageDocument, inputMessagePhoto or inputMessageVideo
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_media(chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageMedia",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the message content caption. Returns the edited message after the edit is completed on the server side
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `caption` - New message content caption; 0-getOption("message_caption_length_max") characters; pass null to remove caption
/// * `show_caption_above_media` - Pass true to show the caption above the media; otherwise, the caption will be shown below the media. May be true only for animation, photo, and video messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_caption(chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, caption: Option<crate::types::FormattedText>, show_caption_above_media: bool, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageCaption",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
        "caption": caption,
        "show_caption_above_media": show_caption_above_media,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_reply_markup(chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageReplyMarkup",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits the text of an inline text or game message sent via the bot; for bots only
///
/// # Arguments
///
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `input_message_content` - New text content of the message. Must be of type inputMessageText or inputMessageRichMessage; file upload isn't supported
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_text(inline_message_id: String, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editInlineMessageText",
        "inline_message_id": inline_message_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the content of a live location in an inline message sent via a bot; for bots only
///
/// # Arguments
///
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `location` - New live location of the message; pass null to stop sharing the live location. If the new live_period isn't set to 0x7FFFFFFF,
/// then it must not exceed the current live_period by more than a day, and the live location expiration date must remain in the next 90 days
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_live_location(inline_message_id: String, reply_markup: Option<crate::enums::ReplyMarkup>, location: Option<crate::types::LiveLocation>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editInlineMessageLiveLocation",
        "inline_message_id": inline_message_id,
        "reply_markup": reply_markup,
        "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the media content of a message with a text, an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only
///
/// # Arguments
///
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessageDocument, inputMessagePhoto or inputMessageVideo
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_media(inline_message_id: String, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editInlineMessageMedia",
        "inline_message_id": inline_message_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the caption of an inline message sent via a bot; for bots only
///
/// # Arguments
///
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `caption` - New message content caption; pass null to remove caption; 0-getOption("message_caption_length_max") characters
/// * `show_caption_above_media` - Pass true to show the caption above the media; otherwise, the caption will be shown below the media. May be true only for animation, photo, and video messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_caption(inline_message_id: String, reply_markup: Option<crate::enums::ReplyMarkup>, caption: Option<crate::types::FormattedText>, show_caption_above_media: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editInlineMessageCaption",
        "inline_message_id": inline_message_id,
        "reply_markup": reply_markup,
        "caption": caption,
        "show_caption_above_media": show_caption_above_media,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the reply markup of an inline message sent via a bot; for bots only
///
/// # Arguments
///
/// * `inline_message_id` - Inline message identifier
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_inline_message_reply_markup(inline_message_id: String, reply_markup: Option<crate::enums::ReplyMarkup>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editInlineMessageReplyMarkup",
        "inline_message_id": inline_message_id,
        "reply_markup": reply_markup,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the text, media, or reply markup of an ephemeral message sent by the bot; for bots only
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `receiver_user_id` - Identifier of the user who received the message
/// * `ephemeral_message_id` - Identifier of the ephemeral message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `input_message_content` - New content of the message; pass null to edit only reply markup. Must be one of the following types: inputMessageText, inputMessageAnimation,
/// inputMessageAudio, inputMessageDocument, inputMessagePhoto, inputMessageRichMessage, inputMessageSticker, inputMessageVideo, inputMessageVideoNote, inputMessageVoiceNote
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_ephemeral_message(chat_id: i64, receiver_user_id: i64, ephemeral_message_id: i32, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: Option<crate::enums::InputMessageContent>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editEphemeralMessage",
        "chat_id": chat_id,
        "receiver_user_id": receiver_user_id,
        "ephemeral_message_id": ephemeral_message_id,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the caption and reply markup of an ephemeral message sent by the bot; for bots only
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `receiver_user_id` - Identifier of the user who received the message
/// * `ephemeral_message_id` - Identifier of the ephemeral message
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `caption` - New message content caption; pass null to remove caption; 0-getOption("message_caption_length_max") characters
/// * `show_caption_above_media` - Pass true to show the caption above the media; otherwise, the caption will be shown below the media. May be true only for animation, photo, and video messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_ephemeral_message_caption(chat_id: i64, receiver_user_id: i64, ephemeral_message_id: i32, reply_markup: Option<crate::enums::ReplyMarkup>, caption: Option<crate::types::FormattedText>, show_caption_above_media: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editEphemeralMessageCaption",
        "chat_id": chat_id,
        "receiver_user_id": receiver_user_id,
        "ephemeral_message_id": ephemeral_message_id,
        "reply_markup": reply_markup,
        "caption": caption,
        "show_caption_above_media": show_caption_above_media,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_edit_scheduling_state to check whether the message is suitable
/// * `scheduling_state` - The new message scheduling state; pass null to send the message immediately. Must be null for messages in the state messageSchedulingStateSendWhenVideoProcessed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_scheduling_state(chat_id: i64, message_id: i64, scheduling_state: Option<crate::enums::MessageSchedulingState>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editMessageSchedulingState",
        "chat_id": chat_id,
        "message_id": message_id,
        "scheduling_state": scheduling_state,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes message ephemeral content and reverts message state to the original
///
/// # Arguments
///
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_message_ephemeral_content(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteMessageEphemeralContent",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the fact-check of a message. Can be only used if messageProperties.can_set_fact_check == true
///
/// # Arguments
///
/// * `chat_id` - The channel chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `text` - New text of the fact-check; 0-getOption("fact_check_length_max") characters; pass null to remove it. Only Bold, Italic, and TextUrl entities with https:t.me/ links are supported
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_message_fact_check(chat_id: i64, message_id: i64, text: Option<crate::types::FormattedText>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMessageFactCheck",
        "chat_id": chat_id,
        "message_id": message_id,
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Loads quick reply messages that can be sent by a given quick reply shortcut. The loaded messages will be sent through updateQuickReplyShortcutMessages
///
/// # Arguments
///
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_quick_reply_shortcut_messages(shortcut_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadQuickReplyShortcutMessages",
        "shortcut_id": shortcut_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes specified quick reply messages
///
/// # Arguments
///
/// * `shortcut_id` - Unique identifier of the quick reply shortcut to which the messages belong
/// * `message_ids` - Unique identifiers of the messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_quick_reply_shortcut_messages(shortcut_id: i32, message_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteQuickReplyShortcutMessages",
        "shortcut_id": shortcut_id,
        "message_ids": message_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a message to a quick reply shortcut. If shortcut doesn't exist and there are less than getOption("quick_reply_shortcut_count_max") shortcuts, then a new shortcut is created.
/// The shortcut must not contain more than getOption("quick_reply_shortcut_message_count_max") messages after adding the new message. Returns the added message
///
/// # Arguments
///
/// * `shortcut_name` - Name of the target shortcut
/// * `reply_to_message_id` - Identifier of a quick reply message in the same shortcut to be replied; pass 0 if none
/// * `input_message_content` - The content of the message to be added; inputMessagePaidMedia, inputMessageForwarded and inputMessageLiveLocation
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::QuickReplyMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_quick_reply_shortcut_message(shortcut_name: String, reply_to_message_id: i64, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<crate::enums::QuickReplyMessage, crate::types::Error> {
    let request = json!({
        "@type": "addQuickReplyShortcutMessage",
        "shortcut_name": shortcut_name,
        "reply_to_message_id": reply_to_message_id,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds 2-10 messages grouped together into an album to a quick reply shortcut. Currently, only audio, document, photo and video messages can be grouped into an album.
/// Documents and audio files can be only grouped in an album with messages of the same type. Returns sent messages
///
/// # Arguments
///
/// * `shortcut_name` - Name of the target shortcut
/// * `reply_to_message_id` - Identifier of a quick reply message in the same shortcut to be replied; pass 0 if none
/// * `input_message_contents` - Contents of messages to be sent. At most 10 messages can be added to an album. All messages must have the same value of show_caption_above_media
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::QuickReplyMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_quick_reply_shortcut_message_album(shortcut_name: String, reply_to_message_id: i64, input_message_contents: Vec<crate::enums::InputMessageContent>, client_id: i32) -> Result<crate::enums::QuickReplyMessages, crate::types::Error> {
    let request = json!({
        "@type": "addQuickReplyShortcutMessageAlbum",
        "shortcut_name": shortcut_name,
        "reply_to_message_id": reply_to_message_id,
        "input_message_contents": input_message_contents,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Re-adds quick reply messages which failed to add. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed.
/// If a message is re-added, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-added, null will be returned instead of the message
///
/// # Arguments
///
/// * `shortcut_name` - Name of the target shortcut
/// * `message_ids` - Identifiers of the quick reply messages to re-add. Message identifiers must be in a strictly increasing order
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::QuickReplyMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn readd_quick_reply_shortcut_messages(shortcut_name: String, message_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::QuickReplyMessages, crate::types::Error> {
    let request = json!({
        "@type": "readdQuickReplyShortcutMessages",
        "shortcut_name": shortcut_name,
        "message_ids": message_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Asynchronously edits the text, media or caption of a quick reply message. Use quickReplyMessage.can_be_edited to check whether a message can be edited.
/// Media message can be edited only to a media message. Checklist messages can be edited only to a checklist message.
/// The type of message content in an album can't be changed with exception of replacing a photo with a video or vice versa
///
/// # Arguments
///
/// * `shortcut_id` - Unique identifier of the quick reply shortcut with the message
/// * `message_id` - Identifier of the message
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessageChecklist, inputMessageDocument, inputMessagePhoto, inputMessageRichMessage, inputMessageText, or inputMessageVideo
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_quick_reply_message(shortcut_id: i32, message_id: i64, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editQuickReplyMessage",
        "shortcut_id": shortcut_id,
        "message_id": message_id,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Loads welcome messages of a chat; requires can_send_welcome_messages administrator right in the chat. The loaded messages will be sent through updateChatWelcomeMessages
///
/// # Arguments
///
/// * `chat_id` - The identifier of the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_chat_welcome_messages(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadChatWelcomeMessages",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a message to the list of welcome messages of a chat; requires can_send_welcome_messages administrator right in the chat. There can be up to getOption("welcome_message_count_max") welcome messages in a chat
///
/// # Arguments
///
/// * `chat_id` - The identifier of the chat
/// * `input_message_content` - The content of the message to be sent. Must be one of the following types: inputMessageText, inputMessageAnimation,
/// inputMessageAudio, inputMessageDocument, inputMessagePhoto, inputMessageRichMessage, inputMessageSticker, inputMessageVideo, inputMessageVideoNote, inputMessageVoiceNote,
/// inputMessageLocation, inputMessageVenue, inputMessageContact
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_welcome_message(chat_id: i64, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addChatWelcomeMessage",
        "chat_id": chat_id,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits a welcome message of a chat; requires can_send_welcome_messages administrator right in the chat
///
/// # Arguments
///
/// * `chat_id` - The identifier of the chat
/// * `welcome_message_id` - The identifier of the welcome message
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageText, inputMessageAnimation,
/// inputMessageAudio, inputMessageDocument, inputMessagePhoto, inputMessageRichMessage, inputMessageSticker, inputMessageVideo, inputMessageVideoNote, inputMessageVoiceNote
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_chat_welcome_message(chat_id: i64, welcome_message_id: i32, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editChatWelcomeMessage",
        "chat_id": chat_id,
        "welcome_message_id": welcome_message_id,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes a welcome message of a chat; requires can_send_welcome_messages administrator right in the chat
///
/// # Arguments
///
/// * `chat_id` - The identifier of the chat
/// * `welcome_message_id` - The identifier of the welcome message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_welcome_message(chat_id: i64, welcome_message_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatWelcomeMessage",
        "chat_id": chat_id,
        "welcome_message_id": welcome_message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all welcome messages of a chat; requires can_send_welcome_messages administrator right in the chat
///
/// # Arguments
///
/// * `chat_id` - The identifier of the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_all_chat_welcome_messages(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteAllChatWelcomeMessages",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns reactions, which can be added to a message. The list can change after updateActiveEmojiReactions, updateChatAvailableReactions for the chat, or updateMessageInteractionInfo for the message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `row_size` - Number of reaction per row, 5-25
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AvailableReactions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_available_reactions(chat_id: i64, message_id: i64, row_size: i32, client_id: i32) -> Result<crate::enums::AvailableReactions, crate::types::Error> {
    let request = json!({
        "@type": "getMessageAvailableReactions",
        "chat_id": chat_id,
        "message_id": message_id,
        "row_size": row_size,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Clears the list of recently used reactions
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
pub async fn clear_recent_reactions(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearRecentReactions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a reaction or a tag to a message. Use getMessageAvailableReactions to receive the list of available reactions for the message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `reaction_type` - Type of the reaction to add. Use addPendingPaidMessageReaction instead to add the paid reaction
/// * `is_big` - Pass true if the reaction is added with a big animation
/// * `update_recent_reactions` - Pass true if the reaction needs to be added to recent reactions; tags are never added to the list of recent reactions
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_message_reaction(chat_id: i64, message_id: i64, reaction_type: crate::enums::ReactionType, is_big: bool, update_recent_reactions: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addMessageReaction",
        "chat_id": chat_id,
        "message_id": message_id,
        "reaction_type": reaction_type,
        "is_big": is_big,
        "update_recent_reactions": update_recent_reactions,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a reaction from a message. A chosen reaction can always be removed
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `reaction_type` - Type of the reaction to remove. The paid reaction can't be removed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_message_reaction(chat_id: i64, message_id: i64, reaction_type: crate::enums::ReactionType, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeMessageReaction",
        "chat_id": chat_id,
        "message_id": message_id,
        "reaction_type": reaction_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all recent reactions added by the specified sender in a chat. Supported only for basic groups and supergroups; requires can_delete_messages administrator right
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `sender_id` - Identifier of the sender of reactions to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_all_recent_message_reactions_from_sender(chat_id: i64, sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteAllRecentMessageReactionsFromSender",
        "chat_id": chat_id,
        "sender_id": sender_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all reactions added by the specified sender on a message
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message containing the reactions. Use messageProperties.can_delete_reactions to check whether the method can be used for a message
/// * `sender_id` - Identifier of the sender of reactions to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_message_reactions_from_sender(chat_id: i64, message_id: i64, sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteMessageReactionsFromSender",
        "chat_id": chat_id,
        "message_id": message_id,
        "sender_id": sender_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of message sender identifiers, which can be used to send a paid reaction in a chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageSenders)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_available_paid_message_reaction_senders(chat_id: i64, client_id: i32) -> Result<crate::enums::MessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getChatAvailablePaidMessageReactionSenders",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds the paid message reaction to a message. Use getMessageAvailableReactions to check whether the reaction is available for the message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `star_count` - Number of Telegram Stars to be used for the reaction. The total number of pending paid reactions must not exceed getOption("paid_reaction_star_count_max")
/// * `r#type` - Type of the paid reaction; pass null if the user didn't choose reaction type explicitly, for example, the reaction is set from the message bubble
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_pending_paid_message_reaction(chat_id: i64, message_id: i64, star_count: i64, r#type: Option<crate::enums::PaidReactionType>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addPendingPaidMessageReaction",
        "chat_id": chat_id,
        "message_id": message_id,
        "star_count": star_count,
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Applies all pending paid reactions on a message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn commit_pending_paid_message_reactions(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "commitPendingPaidMessageReactions",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes all pending paid reactions on a message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_pending_paid_message_reactions(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removePendingPaidMessageReactions",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes type of paid message reaction of the current user on a message. The message must have paid reaction added by the current user
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `r#type` - New type of the paid reaction
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_paid_message_reaction_type(chat_id: i64, message_id: i64, r#type: crate::enums::PaidReactionType, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPaidMessageReactionType",
        "chat_id": chat_id,
        "message_id": message_id,
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets reactions on a message; for bots only
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `reaction_types` - Types of the reaction to set; pass an empty list to remove the reactions
/// * `is_big` - Pass true if the reactions are added with a big animation
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_message_reactions(chat_id: i64, message_id: i64, reaction_types: Vec<crate::enums::ReactionType>, is_big: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMessageReactions",
        "chat_id": chat_id,
        "message_id": message_id,
        "reaction_types": reaction_types,
        "is_big": is_big,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns reactions added for a message, along with their sender
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message. Use message.interaction_info.reactions.can_get_added_reactions to check whether added reactions can be received for the message
/// * `reaction_type` - Type of the reactions to return; pass null to return all added reactions; reactionTypePaid isn't supported
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of reactions to be returned; must be positive and can't be greater than 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AddedReactions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_added_reactions(chat_id: i64, message_id: i64, reaction_type: Option<crate::enums::ReactionType>, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::AddedReactions, crate::types::Error> {
    let request = json!({
        "@type": "getMessageAddedReactions",
        "chat_id": chat_id,
        "message_id": message_id,
        "reaction_type": reaction_type,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes type of default reaction for the current user
///
/// # Arguments
///
/// * `reaction_type` - New type of the default reaction. The paid reaction can't be set as default
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_default_reaction_type(reaction_type: crate::enums::ReactionType, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDefaultReactionType",
        "reaction_type": reaction_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns tags used in Saved Messages or a Saved Messages topic
///
/// # Arguments
///
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which tags will be returned; pass 0 to get all Saved Messages tags
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::SavedMessagesTags)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_messages_tags(saved_messages_topic_id: i64, client_id: i32) -> Result<crate::enums::SavedMessagesTags, crate::types::Error> {
    let request = json!({
        "@type": "getSavedMessagesTags",
        "saved_messages_topic_id": saved_messages_topic_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes label of a Saved Messages tag; for Telegram Premium users only
///
/// # Arguments
///
/// * `tag` - The tag which label will be changed
/// * `label` - New label for the tag; 0-12 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_saved_messages_tag_label(tag: crate::enums::ReactionType, label: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSavedMessagesTagLabel",
        "tag": tag,
        "label": label,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a message effect. Returns a 404 error if the effect is not found
///
/// # Arguments
///
/// * `effect_id` - Unique identifier of the effect
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageEffect)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_effect(effect_id: i64, client_id: i32) -> Result<crate::enums::MessageEffect, crate::types::Error> {
    let request = json!({
        "@type": "getMessageEffect",
        "effect_id": effect_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns all entities (mentions, hashtags, cashtags, bot commands, bank card numbers, URLs, and email addresses) found in the text. Can be called synchronously
///
/// # Arguments
///
/// * `text` - The text in which to look for entities
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TextEntities)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_text_entities(text: String, client_id: i32) -> Result<crate::enums::TextEntities, crate::types::Error> {
    let request = json!({
        "@type": "getTextEntities",
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Parses Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, BlockQuote, ExpandableBlockQuote, Code, Pre, PreCode, TextUrl,
/// MentionName, and DateTime entities from a marked-up text. Can be called synchronously
///
/// # Arguments
///
/// * `text` - The text to parse
/// * `parse_mode` - Text parse mode
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FormattedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn parse_text_entities(text: String, parse_mode: crate::enums::TextParseMode, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "parseTextEntities",
        "text": text,
        "parse_mode": parse_mode,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Replaces text entities with Markdown formatting in a human-friendly format. Entities that can't be represented in Markdown unambiguously are kept as is. Can be called synchronously
///
/// # Arguments
///
/// * `text` - The text
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FormattedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_markdown_text(text: crate::types::FormattedText, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "getMarkdownText",
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Saves an inline message to be sent by the given user; for bots only
///
/// # Arguments
///
/// * `user_id` - Identifier of the user
/// * `result` - The description of the message
/// * `chat_types` - Types of the chats to which the message can be sent
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PreparedInlineMessageId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn save_prepared_inline_message(user_id: i64, result: crate::enums::InputInlineQueryResult, chat_types: crate::types::TargetChatTypes, client_id: i32) -> Result<crate::enums::PreparedInlineMessageId, crate::types::Error> {
    let request = json!({
        "@type": "savePreparedInlineMessage",
        "user_id": user_id,
        "result": result,
        "chat_types": chat_types,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Saves an inline message to be sent by the given user
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the bot that created the message
/// * `prepared_message_id` - Identifier of the prepared message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PreparedInlineMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_prepared_inline_message(bot_user_id: i64, prepared_message_id: String, client_id: i32) -> Result<crate::enums::PreparedInlineMessage, crate::types::Error> {
    let request = json!({
        "@type": "getPreparedInlineMessage",
        "bot_user_id": bot_user_id,
        "prepared_message_id": prepared_message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a replyMarkupForceReply reply markup has been used or dismissed
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - The message identifier of the used keyboard
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_reply_markup(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatReplyMarkup",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a draft for a being generated text message; for bots only
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - The forum topic identifier in which the message will be sent; pass 0 if none
/// * `draft_id` - Unique identifier of the draft
/// * `can_stop` - Pass true to show the user a button to stop further drafts
/// * `keep_on_stop` - Pass true to keep the current draft when the user stops further generation
/// * `text` - Draft text of the message; pass null to show a "Thinking..." placeholder
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_text_message_draft(chat_id: i64, forum_topic_id: i32, draft_id: i64, can_stop: bool, keep_on_stop: bool, text: Option<crate::types::FormattedText>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendTextMessageDraft",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "draft_id": draft_id,
        "can_stop": can_stop,
        "keep_on_stop": keep_on_stop,
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a draft for a being generated rich message; for bots only
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - The forum topic identifier in which the message will be sent; pass 0 if none
/// * `draft_id` - Unique identifier of the draft
/// * `can_stop` - Pass true to show the user a button to stop further drafts
/// * `keep_on_stop` - Pass true to keep the current draft when the user stops further generation
/// * `message` - Draft of the message; file upload isn't supported
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_rich_message_draft(chat_id: i64, forum_topic_id: i32, draft_id: i64, can_stop: bool, keep_on_stop: bool, message: crate::types::InputRichMessage, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendRichMessageDraft",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "draft_id": draft_id,
        "can_stop": can_stop,
        "keep_on_stop": keep_on_stop,
        "message": message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Stops a pending message generation by a bot
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat with the bot
/// * `topic_id` - Identifier of the topic in which the action is performed; pass null if none
/// * `draft_id` - Unique identifier of the message draft within the message thread
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn stop_pending_message(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, draft_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "stopPendingMessage",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "draft_id": draft_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that messages are being viewed by the user. Sponsored messages must be marked as viewed only when the entire text of the message is shown on the screen (excluding the button).
/// Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_ids` - The identifiers of the messages being viewed
/// * `source` - Source of the message view; pass null to guess the source based on chat open state
/// * `force_read` - Pass true to mark as read the specified messages even if the chat is closed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn view_messages(chat_id: i64, message_ids: Vec<i64>, source: Option<crate::enums::MessageSource>, force_read: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewMessages",
        "chat_id": chat_id,
        "message_ids": message_ids,
        "source": source,
        "force_read": force_read,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message).
/// An updateMessageContentOpened update will be generated if something has changed
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the message
/// * `message_id` - Identifier of the message with the opened content
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn open_message_content(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openMessageContent",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib about details of a message view by the user from a chat, a message thread or a forum topic history. The method must be called if
/// the message wasn't seen for more than 300 milliseconds, the viewport was destroyed, or the total view duration exceeded 5 minutes
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - The identifier of the message being viewed
/// * `time_in_view_ms` - The amount of time the message was seen by at least 1 pixel; in milliseconds
/// * `active_time_in_view_ms` - The amount of time the message was seen by at least 1 pixel within 15 seconds after any action from the user; in milliseconds
/// * `height_to_viewport_ratio_per_mille` - The ratio of the post height to the viewport height in 1/1000 fractions
/// * `seen_range_ratio_per_mille` - The ratio of the viewed post height to the full post height in 1/1000 fractions; 0-1000
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_message_view_metrics(chat_id: i64, message_id: i64, time_in_view_ms: i32, active_time_in_view_ms: i32, height_to_viewport_ratio_per_mille: i32, seen_range_ratio_per_mille: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendMessageViewMetrics",
        "chat_id": chat_id,
        "message_id": message_id,
        "time_in_view_ms": time_in_view_ms,
        "active_time_in_view_ms": active_time_in_view_ms,
        "height_to_viewport_ratio_per_mille": height_to_viewport_ratio_per_mille,
        "seen_range_ratio_per_mille": seen_range_ratio_per_mille,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Marks all reactions in a chat as read
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn read_all_chat_reactions(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllChatReactions",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the message auto-delete or self-destruct (for secret chats) time in a chat. Requires change_info administrator right in basic groups, supergroups and channels.
/// Message auto-delete time can't be changed in a chat with the current user (Saved Messages) and the chat 777000 (Telegram).
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_auto_delete_time` - New time value, in seconds; unless the chat is secret, it must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_message_auto_delete_time(chat_id: i64, message_auto_delete_time: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatMessageAutoDeleteTime",
        "chat_id": chat_id,
        "message_auto_delete_time": message_auto_delete_time,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the draft message in a chat or a topic
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `topic_id` - Topic in which the draft will be changed; pass null to change the draft for the chat itself
/// * `draft_message` - New draft message; pass null to remove the draft
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_draft_message(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, draft_message: Option<crate::types::DraftMessage>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDraftMessage",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "draft_message": draft_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes reactions, available in a chat. Available for basic groups, supergroups, and channels. Requires can_change_info member right
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `available_reactions` - Reactions available in the chat. All explicitly specified emoji reactions must be active. In channel chats up to the chat's boost level custom emoji reactions can be explicitly specified
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_available_reactions(chat_id: i64, available_reactions: crate::enums::ChatAvailableReactions, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatAvailableReactions",
        "chat_id": chat_id,
        "available_reactions": available_reactions,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes direct messages group settings for a channel chat; requires owner privileges in the chat
///
/// # Arguments
///
/// * `chat_id` - Identifier of the channel chat
/// * `is_enabled` - Pass true if the direct messages group is enabled for the channel chat; pass false otherwise
/// * `paid_message_star_count` - The new number of Telegram Stars that must be paid for each message that is sent to the direct messages chat unless the sender is an administrator of the channel chat; 0-getOption("paid_message_star_count_max").
/// The channel will receive getOption("paid_message_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for message sending. Requires supergroupFullInfo.can_enable_paid_messages for positive amounts
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_direct_messages_group(chat_id: i64, is_enabled: bool, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDirectMessagesGroup",
        "chat_id": chat_id,
        "is_enabled": is_enabled,
        "paid_message_star_count": paid_message_star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Pins a message in a chat. A message can be pinned only if messageProperties.can_be_pinned
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `message_id` - Identifier of the new pinned message
/// * `disable_notification` - Pass true to disable notification about the pinned message. Notifications are always disabled in channels and private chats
/// * `only_for_self` - Pass true to pin the message only for self; private chats only
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn pin_chat_message(chat_id: i64, message_id: i64, disable_notification: bool, only_for_self: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "pinChatMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "disable_notification": disable_notification,
        "only_for_self": only_for_self,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a pinned message from a chat; requires can_pin_messages member right if the chat is a basic group or supergroup, or can_edit_messages administrator right if the chat is a channel
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `message_id` - Identifier of the removed pinned message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn unpin_chat_message(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "unpinChatMessage",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes all pinned messages from a chat; requires can_pin_messages member right if the chat is a basic group or supergroup, or can_edit_messages administrator right if the chat is a channel
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn unpin_all_chat_messages(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "unpinAllChatMessages",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Clears message drafts in all chats
///
/// # Arguments
///
/// * `exclude_secret_chats` - Pass true to keep local message drafts in secret chats
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn clear_all_draft_messages(exclude_secret_chats: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearAllDraftMessages",
        "exclude_secret_chats": exclude_secret_chats,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a confirmation text to be shown to the user before starting message import
///
/// # Arguments
///
/// * `chat_id` - Identifier of a chat to which the messages will be imported. It must be an identifier of a private chat with a mutual contact or an identifier of a supergroup chat with can_change_info member right
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_import_confirmation_text(chat_id: i64, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getMessageImportConfirmationText",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Imports messages exported from another application
///
/// # Arguments
///
/// * `chat_id` - Identifier of a chat to which the messages will be imported. It must be an identifier of a private chat with a mutual contact or an identifier of a supergroup chat with can_change_info member right
/// * `message_file` - File with messages to import. Only inputFileLocal and inputFileGenerated are supported. The file must not be previously uploaded
/// * `attached_files` - Files used in the imported messages. Only inputFileLocal and inputFileGenerated are supported. The files must not be previously uploaded
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn import_messages(chat_id: i64, message_file: crate::enums::InputFile, attached_files: Vec<crate::enums::InputFile>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "importMessages",
        "chat_id": chat_id,
        "message_file": message_file,
        "attached_files": attached_files,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the block list of a message sender. Currently, only users and supergroup chats can be blocked
///
/// # Arguments
///
/// * `sender_id` - Identifier of a message sender to block/unblock
/// * `block_list` - New block list for the message sender; pass null to unblock the message sender
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_message_sender_block_list(sender_id: crate::enums::MessageSender, block_list: Option<crate::enums::BlockList>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMessageSenderBlockList",
        "sender_id": sender_id,
        "block_list": block_list,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Blocks an original sender of a message in the Replies chat
///
/// # Arguments
///
/// * `message_id` - The identifier of an incoming message in the Replies chat
/// * `delete_message` - Pass true to delete the message
/// * `delete_all_messages` - Pass true to delete all messages from the same sender
/// * `report_spam` - Pass true to report the sender to the Telegram moderators
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn block_message_sender_from_replies(message_id: i64, delete_message: bool, delete_all_messages: bool, report_spam: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "blockMessageSenderFromReplies",
        "message_id": message_id,
        "delete_message": delete_message,
        "delete_all_messages": delete_all_messages,
        "report_spam": report_spam,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns users and chats that were blocked by the current user
///
/// # Arguments
///
/// * `block_list` - Block list from which to return users
/// * `offset` - Number of users and chats to skip in the result; must be non-negative
/// * `limit` - The maximum number of users and chats to return; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageSenders)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_blocked_message_senders(block_list: crate::enums::BlockList, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::MessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getBlockedMessageSenders",
        "block_list": block_list,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Toggles whether the current user has sponsored messages enabled. The setting has no effect for users without Telegram Premium for which sponsored messages are always enabled
///
/// # Arguments
///
/// * `has_sponsored_messages_enabled` - Pass true to enable sponsored messages for the current user; false to disable them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_has_sponsored_messages_enabled(has_sponsored_messages_enabled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleHasSponsoredMessagesEnabled",
        "has_sponsored_messages_enabled": has_sponsored_messages_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether sender signature or link to the account is added to sent messages in a channel; requires can_change_info member right
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the channel
/// * `sign_messages` - New value of sign_messages
/// * `show_message_sender` - New value of show_message_sender
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_sign_messages(supergroup_id: i64, sign_messages: bool, show_message_sender: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupSignMessages",
        "supergroup_id": supergroup_id,
        "sign_messages": sign_messages,
        "show_message_sender": show_message_sender,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether joining is mandatory to send messages to a discussion supergroup; requires can_restrict_members administrator right
///
/// # Arguments
///
/// * `supergroup_id` - Identifier of the supergroup that isn't a broadcast group
/// * `join_to_send_messages` - New value of join_to_send_messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_join_to_send_messages(supergroup_id: i64, join_to_send_messages: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupJoinToSendMessages",
        "supergroup_id": supergroup_id,
        "join_to_send_messages": join_to_send_messages,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether sponsored messages are shown in the channel chat; requires owner privileges in the channel. The chat must have at least chatBoostFeatures.min_sponsored_message_disable_boost_level boost level to disable sponsored messages
///
/// # Arguments
///
/// * `supergroup_id` - The identifier of the channel
/// * `can_have_sponsored_messages` - The new value of can_have_sponsored_messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_can_have_sponsored_messages(supergroup_id: i64, can_have_sponsored_messages: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupCanHaveSponsoredMessages",
        "supergroup_id": supergroup_id,
        "can_have_sponsored_messages": can_have_sponsored_messages,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Allows the specified user to send unpaid private messages to the current user by adding a rule to userPrivacySettingAllowUnpaidMessages
///
/// # Arguments
///
/// * `user_id` - Identifier of the user
/// * `refund_payments` - Pass true to refund the user previously paid messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn allow_unpaid_messages_from_user(user_id: i64, refund_payments: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "allowUnpaidMessagesFromUser",
        "user_id": user_id,
        "refund_payments": refund_payments,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks whether the current user can message another user or try to create a chat with them
///
/// # Arguments
///
/// * `user_id` - Identifier of the other user
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CanSendMessageToUserResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn can_send_message_to_user(user_id: i64, only_local: bool, client_id: i32) -> Result<crate::enums::CanSendMessageToUserResult, crate::types::Error> {
    let request = json!({
        "@type": "canSendMessageToUser",
        "user_id": user_id,
        "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the default message auto-delete time for new chats
///
/// # Arguments
///
/// * `message_auto_delete_time` - New default message auto-delete time; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_default_message_auto_delete_time(message_auto_delete_time: crate::types::MessageAutoDeleteTime, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDefaultMessageAutoDeleteTime",
        "message_auto_delete_time": message_auto_delete_time,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns default message auto-delete time setting for new chats
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageAutoDeleteTime)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_default_message_auto_delete_time(client_id: i32) -> Result<crate::enums::MessageAutoDeleteTime, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultMessageAutoDeleteTime",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Reports reactions set on a message to the Telegram moderators. Reactions on a message can be reported only if messageProperties.can_report_reactions
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Message identifier
/// * `sender_id` - Identifier of the sender, which added the reaction
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_message_reactions(chat_id: i64, message_id: i64, sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportMessageReactions",
        "chat_id": chat_id,
        "message_id": message_id,
        "sender_id": sender_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns detailed statistics about a message. Can be used only if messageProperties.can_get_statistics == true
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Message identifier
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_statistics(chat_id: i64, message_id: i64, is_dark: bool, client_id: i32) -> Result<crate::enums::MessageStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getMessageStatistics",
        "chat_id": chat_id,
        "message_id": message_id,
        "is_dark": is_dark,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns forwarded copies of a channel message to different public channels and public reposts as a story. Can be used only if messageProperties.can_get_statistics == true. For optimal performance, the number of returned messages and stories is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the message
/// * `message_id` - Message identifier
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages and stories to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PublicForwards)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_message_public_forwards(chat_id: i64, message_id: i64, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::PublicForwards, crate::types::Error> {
    let request = json!({
        "@type": "getMessagePublicForwards",
        "chat_id": chat_id,
        "message_id": message_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a message to TDLib internal log. Can be called synchronously
///
/// # Arguments
///
/// * `verbosity_level` - The minimum verbosity level needed for the message to be logged; 0-1023
/// * `text` - Text of a message to log
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_log_message(verbosity_level: i32, text: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addLogMessage",
        "verbosity_level": verbosity_level,
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

