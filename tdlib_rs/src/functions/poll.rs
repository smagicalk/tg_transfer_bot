//!
//! TDLib `poll` domain functions.
//!
//! Types, enums, and functions for polls, quiz questions, and voter answers.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns properties of a poll option. This is an offline method
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `poll_option_id` - Unique identifier of the answer option, which properties will be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PollOptionProperties)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_poll_option_properties(chat_id: i64, message_id: i64, poll_option_id: String, client_id: i32) -> Result<crate::enums::PollOptionProperties, crate::types::Error> {
    let request = json!({
        "@type": "getPollOptionProperties",
        "chat_id": chat_id,
        "message_id": message_id,
        "poll_option_id": poll_option_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds an option to a poll
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll. Use messagePoll.can_add_option to check whether an option can be added
/// * `option` - The new option
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_poll_option(chat_id: i64, message_id: i64, option: crate::types::InputPollOption, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addPollOption",
        "chat_id": chat_id,
        "message_id": message_id,
        "option": option,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes an option from a poll
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll
/// * `option_id` - Unique identifier of the option. Use pollOptionProperties.can_be_deleted to check whether the option can be deleted by the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_poll_option(chat_id: i64, message_id: i64, option_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deletePollOption",
        "chat_id": chat_id,
        "message_id": message_id,
        "option_id": option_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the user answer to a poll
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll
/// * `option_ids` - 0-based identifiers of answer options, chosen by the user. User can choose more than 1 answer option only is the poll allows multiple answers
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_poll_answer(chat_id: i64, message_id: i64, option_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPollAnswer",
        "chat_id": chat_id,
        "message_id": message_id,
        "option_ids": option_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns message senders voted for the specified option in a poll; use poll.can_get_voters to check whether the method can be used.
/// For optimal performance, the number of returned users is chosen by TDLib
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll
/// * `option_id` - 0-based identifier of the answer option
/// * `offset` - Number of voters to skip in the result; must be non-negative
/// * `limit` - The maximum number of voters to be returned; must be positive and can't be greater than 50. For optimal performance, the number of returned voters is chosen by TDLib and can be smaller than the specified limit, even if the end of the voter list has not been reached
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PollVoters)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_poll_voters(chat_id: i64, message_id: i64, option_id: i32, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::PollVoters, crate::types::Error> {
    let request = json!({
        "@type": "getPollVoters",
        "chat_id": chat_id,
        "message_id": message_id,
        "option_id": option_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns statistics of poll votes in a poll
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll. Use messageProperties.can_get_poll_vote_statistics to check whether the method can be used for a message
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PollVoteStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_poll_vote_statistics(chat_id: i64, message_id: i64, is_dark: bool, client_id: i32) -> Result<crate::enums::PollVoteStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getPollVoteStatistics",
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

/// Stops a poll
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll. Use messageProperties.can_be_edited to check whether the poll can be stopped
/// * `reply_markup` - The new message reply markup; pass null if none; for bots only
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn stop_poll(chat_id: i64, message_id: i64, reply_markup: Option<crate::enums::ReplyMarkup>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "stopPoll",
        "chat_id": chat_id,
        "message_id": message_id,
        "reply_markup": reply_markup,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Marks all poll votes in a chat as read
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
pub async fn read_all_chat_poll_votes(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "readAllChatPollVotes",
        "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

