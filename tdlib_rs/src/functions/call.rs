//!
//! TDLib `call` domain functions.
//!
//! Types, enums, and functions for 1-on-1 calls, group calls, and live video chats.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns information about a message, if it is available without sending network request. Returns a 404 error if message isn't available locally. This is an offline method
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
pub async fn get_message_locally(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getMessageLocally",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a message with the callback button that originated a callback query; for bots only
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `message_id` - Message identifier
/// * `callback_query_id` - Identifier of the callback query
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_callback_query_message(chat_id: i64, message_id: i64, callback_query_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getCallbackQueryMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "callback_query_id": callback_query_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for call and group call messages. Returns the results in reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
///
/// # Arguments
///
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `only_missed` - Pass true to search only for messages with missed/declined calls
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundMessages)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_call_messages(offset: String, limit: i32, only_missed: bool, client_id: i32) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
        "@type": "searchCallMessages",
        "offset": offset,
        "limit": limit,
        "only_missed": only_missed,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes all call messages
///
/// # Arguments
///
/// * `revoke` - Pass true to delete the messages for all users
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_all_call_messages(revoke: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteAllCallMessages",
        "revoke": revoke,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits the message from which a callback query has originated with an ephemeral message; for bots only
///
/// # Arguments
///
/// * `callback_query_id` - Identifier of the callback query
/// * `protect_content` - Pass true if the content of the message must be protected from forwarding and saving
/// * `reply_markup` - The new message reply markup; pass null if none
/// * `input_message_content` - New content of the message. Must be one of the following types: inputMessageText, inputMessageAnimation,
/// inputMessageAudio, inputMessageDocument, inputMessagePhoto, inputMessageRichMessage, inputMessageSticker, inputMessageVideo, inputMessageVideoNote, inputMessageVoiceNote
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_callback_query_message(callback_query_id: i64, protect_content: bool, reply_markup: Option<crate::enums::ReplyMarkup>, input_message_content: crate::enums::InputMessageContent, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editCallbackQueryMessage",
        "callback_query_id": callback_query_id,
        "protect_content": protect_content,
        "reply_markup": reply_markup,
        "input_message_content": input_message_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message from which the query originated. The message must not be scheduled
/// * `payload` - Query payload
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CallbackQueryAnswer)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_callback_query_answer(chat_id: i64, message_id: i64, payload: crate::enums::CallbackQueryPayload, client_id: i32) -> Result<crate::enums::CallbackQueryAnswer, crate::types::Error> {
    let request = json!({
        "@type": "getCallbackQueryAnswer",
        "chat_id": chat_id,
        "message_id": message_id,
        "payload": payload,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the result of a callback query; for bots only
///
/// # Arguments
///
/// * `callback_query_id` - Identifier of the callback query
/// * `text` - Text of the answer
/// * `show_alert` - Pass true to show an alert to the user instead of a toast notification
/// * `url` - URL to be opened
/// * `cache_time` - Time during which the result of the query can be cached, in seconds
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_callback_query(callback_query_id: i64, text: String, show_alert: bool, url: String, cache_time: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "answerCallbackQuery",
        "callback_query_id": callback_query_id,
        "text": text,
        "show_alert": show_alert,
        "url": url,
        "cache_time": cache_time,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Creates a new call
///
/// # Arguments
///
/// * `user_id` - Identifier of the user to be called
/// * `protocol` - The call protocols supported by the application
/// * `is_video` - Pass true to create a video call
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CallId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_call(user_id: i64, protocol: crate::types::CallProtocol, is_video: bool, client_id: i32) -> Result<crate::enums::CallId, crate::types::Error> {
    let request = json!({
        "@type": "createCall",
        "user_id": user_id,
        "protocol": protocol,
        "is_video": is_video,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Accepts an incoming call
///
/// # Arguments
///
/// * `call_id` - Call identifier
/// * `protocol` - The call protocols supported by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn accept_call(call_id: i32, protocol: crate::types::CallProtocol, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "acceptCall",
        "call_id": call_id,
        "protocol": protocol,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends call signaling data
///
/// # Arguments
///
/// * `call_id` - Call identifier
/// * `data` - The data
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_call_signaling_data(call_id: i32, data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallSignalingData",
        "call_id": call_id,
        "data": data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Discards a call
///
/// # Arguments
///
/// * `call_id` - Call identifier
/// * `is_disconnected` - Pass true if the user was disconnected
/// * `invite_link` - If the call was upgraded to a group call, pass invite link to the group call
/// * `duration` - The call duration, in seconds
/// * `is_video` - Pass true if the call was a video call
/// * `connection_id` - Identifier of the connection used during the call
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn discard_call(call_id: i32, is_disconnected: bool, invite_link: String, duration: i32, is_video: bool, connection_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "discardCall",
        "call_id": call_id,
        "is_disconnected": is_disconnected,
        "invite_link": invite_link,
        "duration": duration,
        "is_video": is_video,
        "connection_id": connection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a call rating
///
/// # Arguments
///
/// * `call_id` - Call identifier
/// * `rating` - Call rating; 1-5
/// * `comment` - An optional user comment if the rating is less than 5
/// * `problems` - List of the exact types of problems with the call, specified by the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_call_rating(call_id: crate::enums::InputCall, rating: i32, comment: String, problems: Vec<crate::enums::CallProblem>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallRating",
        "call_id": call_id,
        "rating": rating,
        "comment": comment,
        "problems": problems,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends debug information for a call to Telegram servers
///
/// # Arguments
///
/// * `call_id` - Call identifier
/// * `debug_information` - Debug information in application-specific format
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_call_debug_information(call_id: crate::enums::InputCall, debug_information: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallDebugInformation",
        "call_id": call_id,
        "debug_information": debug_information,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends log file for a call to Telegram servers
///
/// # Arguments
///
/// * `call_id` - Call identifier
/// * `log_file` - Call log file. Only inputFileLocal and inputFileGenerated are supported
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_call_log(call_id: crate::enums::InputCall, log_file: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallLog",
        "call_id": call_id,
        "log_file": log_file,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of participant identifiers, on whose behalf a video chat in the chat can be joined
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
pub async fn get_video_chat_available_participants(chat_id: i64, client_id: i32) -> Result<crate::enums::MessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getVideoChatAvailableParticipants",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes default participant identifier, on whose behalf a video chat in the chat will be joined
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `default_participant_id` - Default group call participant identifier to join the video chats in the chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_video_chat_default_participant(chat_id: i64, default_participant_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setVideoChatDefaultParticipant",
        "chat_id": chat_id,
        "default_participant_id": default_participant_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Creates a video chat (a group call bound to a chat); for basic groups, supergroups and channels only; requires can_manage_video_chats administrator right
///
/// # Arguments
///
/// * `chat_id` - Identifier of a chat in which the video chat will be created
/// * `title` - Group call title; if empty, chat title will be used
/// * `start_date` - Point in time (Unix timestamp) when the group call is expected to be started by an administrator; 0 to start the video chat immediately. The date must be at least 10 seconds and at most 8 days in the future
/// * `is_rtmp_stream` - Pass true to create an RTMP stream instead of an ordinary video chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GroupCallId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_video_chat(chat_id: i64, title: String, start_date: i32, is_rtmp_stream: bool, client_id: i32) -> Result<crate::enums::GroupCallId, crate::types::Error> {
    let request = json!({
        "@type": "createVideoChat",
        "chat_id": chat_id,
        "title": title,
        "start_date": start_date,
        "is_rtmp_stream": is_rtmp_stream,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a new group call that isn't bound to a chat
///
/// # Arguments
///
/// * `join_parameters` - Parameters to join the call; pass null to only create call link without joining the call
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GroupCallInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_group_call(join_parameters: Option<crate::types::GroupCallJoinParameters>, client_id: i32) -> Result<crate::enums::GroupCallInfo, crate::types::Error> {
    let request = json!({
        "@type": "createGroupCall",
        "join_parameters": join_parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns RTMP URL for streaming to the video chat of a chat; requires can_manage_video_chats administrator right
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
pub async fn get_video_chat_rtmp_url(chat_id: i64, client_id: i32) -> Result<crate::enums::RtmpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getVideoChatRtmpUrl",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Replaces the current RTMP URL for streaming to the video chat of a chat; requires owner privileges in the chat
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
pub async fn replace_video_chat_rtmp_url(chat_id: i64, client_id: i32) -> Result<crate::enums::RtmpUrl, crate::types::Error> {
    let request = json!({
        "@type": "replaceVideoChatRtmpUrl",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a group call
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GroupCall)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_group_call(group_call_id: i32, client_id: i32) -> Result<crate::enums::GroupCall, crate::types::Error> {
    let request = json!({
        "@type": "getGroupCall",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Starts a scheduled video chat
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier of the video chat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn start_scheduled_video_chat(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "startScheduledVideoChat",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether the current user will receive a notification when the video chat starts; for scheduled video chats only
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `enabled_start_notification` - New value of the enabled_start_notification setting
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_video_chat_enabled_start_notification(group_call_id: i32, enabled_start_notification: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleVideoChatEnabledStartNotification",
        "group_call_id": group_call_id,
        "enabled_start_notification": enabled_start_notification,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Joins a regular group call that is not bound to a chat
///
/// # Arguments
///
/// * `input_group_call` - The group call to join
/// * `join_parameters` - Parameters to join the call
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GroupCallInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn join_group_call(input_group_call: crate::enums::InputGroupCall, join_parameters: crate::types::GroupCallJoinParameters, client_id: i32) -> Result<crate::enums::GroupCallInfo, crate::types::Error> {
    let request = json!({
        "@type": "joinGroupCall",
        "input_group_call": input_group_call,
        "join_parameters": join_parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Joins an active video chat. Returns join response payload for tgcalls
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Identifier of a group call participant, which will be used to join the call; pass null to join as self
/// * `join_parameters` - Parameters to join the call
/// * `invite_hash` - Invite hash as received from internalLinkTypeVideoChat
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn join_video_chat(group_call_id: i32, participant_id: Option<crate::enums::MessageSender>, join_parameters: crate::types::GroupCallJoinParameters, invite_hash: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "joinVideoChat",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "join_parameters": join_parameters,
        "invite_hash": invite_hash,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Starts screen sharing in a joined group call; not supported in live stories. Returns join response payload for tgcalls
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `audio_source_id` - Screen sharing audio channel synchronization source identifier; received from tgcalls
/// * `payload` - Group call join payload; received from tgcalls
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn start_group_call_screen_sharing(group_call_id: i32, audio_source_id: i32, payload: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "startGroupCallScreenSharing",
        "group_call_id": group_call_id,
        "audio_source_id": audio_source_id,
        "payload": payload,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Pauses or unpauses screen sharing in a joined group call; not supported in live stories
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `is_paused` - Pass true to pause screen sharing; pass false to unpause it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_screen_sharing_is_paused(group_call_id: i32, is_paused: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallScreenSharingIsPaused",
        "group_call_id": group_call_id,
        "is_paused": is_paused,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Ends screen sharing in a joined group call; not supported in live stories
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
pub async fn end_group_call_screen_sharing(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "endGroupCallScreenSharing",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets title of a video chat; requires groupCall.can_be_managed right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `title` - New group call title; 1-64 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_video_chat_title(group_call_id: i32, title: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setVideoChatTitle",
        "group_call_id": group_call_id,
        "title": title,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether new participants of a video chat can be unmuted only by administrators of the video chat. Requires groupCall.can_toggle_mute_new_participants right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `mute_new_participants` - New value of the mute_new_participants setting
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_video_chat_mute_new_participants(group_call_id: i32, mute_new_participants: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleVideoChatMuteNewParticipants",
        "group_call_id": group_call_id,
        "mute_new_participants": mute_new_participants,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether participants of a group call can send messages there. Requires groupCall.can_toggle_are_messages_allowed right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `are_messages_allowed` - New value of the are_messages_allowed setting
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_are_messages_allowed(group_call_id: i32, are_messages_allowed: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallAreMessagesAllowed",
        "group_call_id": group_call_id,
        "are_messages_allowed": are_messages_allowed,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a message to other participants of a group call. Requires groupCall.can_send_messages right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `text` - Text of the message to send; 1-getOption("group_call_message_text_length_max") characters for non-live-stories; see updateGroupCallMessageLevels for live story restrictions,
/// which depends on paid_message_star_count. Can't contain line feeds for live stories. Can contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities for live stories
/// * `paid_message_star_count` - The number of Telegram Stars the user agreed to pay to send the message; for live stories only; 0-getOption("paid_group_call_message_star_count_max").
/// Must be 0 for messages sent to live stories posted by the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_group_call_message(group_call_id: i32, text: crate::types::FormattedText, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendGroupCallMessage",
        "group_call_id": group_call_id,
        "text": text,
        "paid_message_star_count": paid_message_star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes messages in a group call; for live story calls only. Requires groupCallMessage.can_be_deleted right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `message_ids` - Identifiers of the messages to be deleted
/// * `report_spam` - Pass true to report the messages as spam
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_group_call_messages(group_call_id: i32, message_ids: Vec<i32>, report_spam: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteGroupCallMessages",
        "group_call_id": group_call_id,
        "message_ids": message_ids,
        "report_spam": report_spam,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all messages sent by the specified message sender in a group call; for live story calls only. Requires groupCall.can_delete_messages right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `sender_id` - Identifier of the sender of messages to delete
/// * `report_spam` - Pass true to report the messages as spam
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_group_call_messages_by_sender(group_call_id: i32, sender_id: crate::enums::MessageSender, report_spam: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteGroupCallMessagesBySender",
        "group_call_id": group_call_id,
        "sender_id": sender_id,
        "report_spam": report_spam,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Invites a user to an active group call; for group calls not bound to a chat only. Sends a service message of the type messageGroupCall.
/// The group call can have at most getOption("group_call_participant_count_max") participants
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `user_id` - User identifier
/// * `is_video` - Pass true if the group call is a video call
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::InviteGroupCallParticipantResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn invite_group_call_participant(group_call_id: i32, user_id: i64, is_video: bool, client_id: i32) -> Result<crate::enums::InviteGroupCallParticipantResult, crate::types::Error> {
    let request = json!({
        "@type": "inviteGroupCallParticipant",
        "group_call_id": group_call_id,
        "user_id": user_id,
        "is_video": is_video,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Declines an invitation to an active group call via messageGroupCall. Can be called both by the sender and the receiver of the invitation
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message of the type messageGroupCall
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn decline_group_call_invitation(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "declineGroupCallInvitation",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Bans users from a group call not bound to a chat; requires groupCall.is_owned. Only the owner of the group call can invite the banned users back
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `user_ids` - Identifiers of group call participants to ban; identifiers of unknown users from the update updateGroupCallParticipants can be also passed to the method
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn ban_group_call_participants(group_call_id: i32, user_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "banGroupCallParticipants",
        "group_call_id": group_call_id,
        "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Invites users to an active video chat. Sends a service message of the type messageInviteVideoChatParticipants to the chat bound to the group call
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `user_ids` - User identifiers. At most 10 users can be invited simultaneously
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn invite_video_chat_participants(group_call_id: i32, user_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "inviteVideoChatParticipants",
        "group_call_id": group_call_id,
        "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns invite link to a video chat in a public chat
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `can_self_unmute` - Pass true if the invite link needs to contain an invite hash, passing which to joinVideoChat would allow the invited user to unmute themselves. Requires groupCall.can_be_managed right
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_video_chat_invite_link(group_call_id: i32, can_self_unmute: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getVideoChatInviteLink",
        "group_call_id": group_call_id,
        "can_self_unmute": can_self_unmute,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Revokes invite link for a group call. Requires groupCall.can_be_managed right for video chats or groupCall.is_owned otherwise
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
pub async fn revoke_group_call_invite_link(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "revokeGroupCallInviteLink",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Starts recording of an active group call; for video chats only. Requires groupCall.can_be_managed right
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `title` - Group call recording title; 0-64 characters
/// * `record_video` - Pass true to record a video file instead of an audio file
/// * `use_portrait_orientation` - Pass true to use portrait orientation for video instead of landscape one
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn start_group_call_recording(group_call_id: i32, title: String, record_video: bool, use_portrait_orientation: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "startGroupCallRecording",
        "group_call_id": group_call_id,
        "title": title,
        "record_video": record_video,
        "use_portrait_orientation": use_portrait_orientation,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Ends recording of an active group call; for video chats only. Requires groupCall.can_be_managed right
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
pub async fn end_group_call_recording(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "endGroupCallRecording",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether current user's video is paused
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `is_my_video_paused` - Pass true if the current user's video is paused
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_is_my_video_paused(group_call_id: i32, is_my_video_paused: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallIsMyVideoPaused",
        "group_call_id": group_call_id,
        "is_my_video_paused": is_my_video_paused,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether current user's video is enabled
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `is_my_video_enabled` - Pass true if the current user's video is enabled
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_is_my_video_enabled(group_call_id: i32, is_my_video_enabled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallIsMyVideoEnabled",
        "group_call_id": group_call_id,
        "is_my_video_enabled": is_my_video_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that speaking state of a participant of an active group call has changed. Returns identifier of the participant if it is found
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `audio_source` - Group call participant's synchronization audio source identifier, or 0 for the current user
/// * `is_speaking` - Pass true if the user is speaking
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MessageSender)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_group_call_participant_is_speaking(group_call_id: i32, audio_source: i32, is_speaking: bool, client_id: i32) -> Result<crate::enums::MessageSender, crate::types::Error> {
    let request = json!({
        "@type": "setGroupCallParticipantIsSpeaking",
        "group_call_id": group_call_id,
        "audio_source": audio_source,
        "is_speaking": is_speaking,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Toggles whether a participant of an active group call is muted, unmuted, or allowed to unmute themselves; not supported for live stories
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Participant identifier
/// * `is_muted` - Pass true to mute the user; pass false to unmute them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_participant_is_muted(group_call_id: i32, participant_id: crate::enums::MessageSender, is_muted: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallParticipantIsMuted",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "is_muted": is_muted,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes volume level of a participant of an active group call; not supported for live stories. If the current user can manage the group call or is the owner of the group call,
/// then the participant's volume level will be changed for all users with the default volume level
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Participant identifier
/// * `volume_level` - New participant's volume level; 1-20000 in hundreds of percents
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_group_call_participant_volume_level(group_call_id: i32, participant_id: crate::enums::MessageSender, volume_level: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGroupCallParticipantVolumeLevel",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "volume_level": volume_level,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether a group call participant hand is rased; for video chats only
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Participant identifier
/// * `is_hand_raised` - Pass true if the user's hand needs to be raised. Only self hand can be raised. Requires groupCall.can_be_managed right to lower other's hand
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_participant_is_hand_raised(group_call_id: i32, participant_id: crate::enums::MessageSender, is_hand_raised: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallParticipantIsHandRaised",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "is_hand_raised": is_hand_raised,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about participants of a non-joined group call that is not bound to a chat
///
/// # Arguments
///
/// * `input_group_call` - The group call which participants will be returned
/// * `limit` - The maximum number of participants to return; must be positive
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GroupCallParticipants)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_group_call_participants(input_group_call: crate::enums::InputGroupCall, limit: i32, client_id: i32) -> Result<crate::enums::GroupCallParticipants, crate::types::Error> {
    let request = json!({
        "@type": "getGroupCallParticipants",
        "input_group_call": input_group_call,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Loads more participants of a group call; not supported in live stories. The loaded participants will be received through updates.
/// Use the field groupCall.loaded_all_participants to check whether all participants have already been loaded
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier. The group call must be previously received through getGroupCall and must be joined or being joined
/// * `limit` - The maximum number of participants to load; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_group_call_participants(group_call_id: i32, limit: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadGroupCallParticipants",
        "group_call_id": group_call_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Leaves a group call
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
pub async fn leave_group_call(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "leaveGroupCall",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Ends a group call. Requires groupCall.can_be_managed right for video chats and live stories or groupCall.is_owned otherwise
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
pub async fn end_group_call(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "endGroupCall",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about available streams in a video chat or a live story
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GroupCallStreams)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_group_call_streams(group_call_id: i32, client_id: i32) -> Result<crate::enums::GroupCallStreams, crate::types::Error> {
    let request = json!({
        "@type": "getGroupCallStreams",
        "group_call_id": group_call_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a file with a segment of a video chat or live story in a modified OGG format for audio or MPEG-4 format for video
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier
/// * `time_offset` - Point in time when the stream segment begins; Unix timestamp in milliseconds
/// * `scale` - Segment duration scale; 0-1. Segment's duration is 1000/(2**scale) milliseconds
/// * `channel_id` - Identifier of an audio/video channel to get as received from tgcalls
/// * `video_quality` - Video quality as received from tgcalls; pass null to get the worst available quality
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Data)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_group_call_stream_segment(group_call_id: i32, time_offset: i64, scale: i32, channel_id: i32, video_quality: Option<crate::enums::GroupCallVideoQuality>, client_id: i32) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
        "@type": "getGroupCallStreamSegment",
        "group_call_id": group_call_id,
        "time_offset": time_offset,
        "scale": scale,
        "channel_id": channel_id,
        "video_quality": video_quality,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Encrypts group call data before sending them over network using tgcalls
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier. The call must not be a video chat
/// * `data_channel` - Data channel for which data is encrypted
/// * `data` - Data to encrypt
/// * `unencrypted_prefix_size` - Size of data prefix that must be kept unencrypted
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Data)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn encrypt_group_call_data(group_call_id: i32, data_channel: crate::enums::GroupCallDataChannel, data: String, unencrypted_prefix_size: i32, client_id: i32) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
        "@type": "encryptGroupCallData",
        "group_call_id": group_call_id,
        "data_channel": data_channel,
        "data": data,
        "unencrypted_prefix_size": unencrypted_prefix_size,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Decrypts group call data received by tgcalls
///
/// # Arguments
///
/// * `group_call_id` - Group call identifier. The call must not be a video chat
/// * `participant_id` - Identifier of the group call participant, which sent the data
/// * `data_channel` - Data channel for which data was encrypted; pass null if unknown
/// * `data` - Data to decrypt
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Data)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn decrypt_group_call_data(group_call_id: i32, participant_id: crate::enums::MessageSender, data_channel: Option<crate::enums::GroupCallDataChannel>, data: String, client_id: i32) -> Result<crate::enums::Data, crate::types::Error> {
    let request = json!({
        "@type": "decryptGroupCallData",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "data_channel": data_channel,
        "data": data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Toggles whether a session can accept incoming calls
///
/// # Arguments
///
/// * `session_id` - Session identifier
/// * `can_accept_calls` - Pass true to allow accepting incoming calls by the session; pass false otherwise
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_session_can_accept_calls(session_id: i64, can_accept_calls: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSessionCanAcceptCalls",
        "session_id": session_id,
        "can_accept_calls": can_accept_calls,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Does nothing; for testing only. This is an offline method. Can be called before authorization
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
pub async fn test_call_empty(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "testCallEmpty",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the received string; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `x` - String to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TestString)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_call_string(x: String, client_id: i32) -> Result<crate::enums::TestString, crate::types::Error> {
    let request = json!({
        "@type": "testCallString",
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the received bytes; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `x` - Bytes to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TestBytes)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_call_bytes(x: String, client_id: i32) -> Result<crate::enums::TestBytes, crate::types::Error> {
    let request = json!({
        "@type": "testCallBytes",
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `x` - Vector of numbers to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TestVectorInt)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_call_vector_int(x: Vec<i32>, client_id: i32) -> Result<crate::enums::TestVectorInt, crate::types::Error> {
    let request = json!({
        "@type": "testCallVectorInt",
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `x` - Vector of objects to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TestVectorIntObject)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_call_vector_int_object(x: Vec<crate::types::TestInt>, client_id: i32) -> Result<crate::enums::TestVectorIntObject, crate::types::Error> {
    let request = json!({
        "@type": "testCallVectorIntObject",
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `x` - Vector of strings to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TestVectorString)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_call_vector_string(x: Vec<String>, client_id: i32) -> Result<crate::enums::TestVectorString, crate::types::Error> {
    let request = json!({
        "@type": "testCallVectorString",
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `x` - Vector of objects to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TestVectorStringObject)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_call_vector_string_object(x: Vec<crate::types::TestString>, client_id: i32) -> Result<crate::enums::TestVectorStringObject, crate::types::Error> {
    let request = json!({
        "@type": "testCallVectorStringObject",
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

