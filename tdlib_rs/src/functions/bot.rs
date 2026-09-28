//!
//! TDLib `bot` domain functions.
//!
//! Types, enums, and functions for Telegram Bots, Web Apps, and inline queries.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in
///
/// # Arguments
///
/// * `token` - The bot token
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_bot_token(token: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationBotToken",
        "token": token,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a list of bots similar to the given bot
///
/// # Arguments
///
/// * `bot_user_id` - User identifier of the target bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Users)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_similar_bots(bot_user_id: i64, client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "getBotSimilarBots",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns approximate number of bots similar to the given bot
///
/// # Arguments
///
/// * `bot_user_id` - User identifier of the target bot
/// * `return_local` - Pass true to get the number of bots without sending network requests, or -1 if the number of bots is unknown locally
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Count)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_similar_bot_count(bot_user_id: i64, return_local: bool, client_id: i32) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
        "@type": "getBotSimilarBotCount",
        "bot_user_id": bot_user_id,
        "return_local": return_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that a bot was opened from the list of similar bots
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the original bot, which similar bots were requested
/// * `opened_bot_user_id` - Identifier of the opened bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn open_bot_similar_bot(bot_user_id: i64, opened_bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openBotSimilarBot",
        "bot_user_id": bot_user_id,
        "opened_bot_user_id": opened_bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Invites a bot to a chat (if it is not yet a member) and sends it the /start command; requires can_invite_users member right. Bots can't be invited to a private chat other than the chat with the bot.
/// Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the bot
/// * `chat_id` - Identifier of the target chat
/// * `parameter` - A hidden parameter sent to the bot for deep linking purposes (https:core.telegram.org/bots#deep-linking)
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_bot_start_message(bot_user_id: i64, chat_id: i64, parameter: String, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "sendBotStartMessage",
        "bot_user_id": bot_user_id,
        "chat_id": chat_id,
        "parameter": parameter,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message
///
/// # Arguments
///
/// * `chat_id` - Target chat
/// * `topic_id` - Topic in which the message will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `options` - Options to be used to send the message; pass null to use default options
/// * `query_id` - Identifier of the inline query
/// * `result_id` - Identifier of the inline query result
/// * `hide_via_bot` - Pass true to hide the bot, via which the message is sent. Can be used only for bots getOption("animation_search_bot_username"), getOption("photo_search_bot_username"), and getOption("venue_search_bot_username")
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_inline_query_result_message(chat_id: i64, topic_id: Option<crate::enums::MessageTopic>, reply_to: Option<crate::enums::InputMessageReplyTo>, options: Option<crate::types::MessageSendOptions>, query_id: i64, result_id: String, hide_via_bot: bool, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "sendInlineQueryResultMessage",
        "chat_id": chat_id,
        "topic_id": topic_id,
        "reply_to": reply_to,
        "options": options,
        "query_id": query_id,
        "result_id": result_id,
        "hide_via_bot": hide_via_bot,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a message to a quick reply shortcut via inline bot. If shortcut doesn't exist and there are less than getOption("quick_reply_shortcut_count_max") shortcuts, then a new shortcut is created.
/// The shortcut must not contain more than getOption("quick_reply_shortcut_message_count_max") messages after adding the new message. Returns the added message
///
/// # Arguments
///
/// * `shortcut_name` - Name of the target shortcut
/// * `reply_to_message_id` - Identifier of a quick reply message in the same shortcut to be replied; pass 0 if none
/// * `query_id` - Identifier of the inline query
/// * `result_id` - Identifier of the inline query result
/// * `hide_via_bot` - Pass true to hide the bot, via which the message is sent. Can be used only for bots getOption("animation_search_bot_username"), getOption("photo_search_bot_username"), and getOption("venue_search_bot_username")
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::QuickReplyMessage)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_quick_reply_shortcut_inline_query_result_message(shortcut_name: String, reply_to_message_id: i64, query_id: i64, result_id: String, hide_via_bot: bool, client_id: i32) -> Result<crate::enums::QuickReplyMessage, crate::types::Error> {
    let request = json!({
        "@type": "addQuickReplyShortcutInlineQueryResultMessage",
        "shortcut_name": shortcut_name,
        "reply_to_message_id": reply_to_message_id,
        "query_id": query_id,
        "result_id": result_id,
        "hide_via_bot": hide_via_bot,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Shares users after pressing a keyboardButtonTypeRequestUsers button with the bot
///
/// # Arguments
///
/// * `source` - Source of the button
/// * `button_id` - Identifier of the button
/// * `shared_user_ids` - Identifiers of the shared users
/// * `only_check` - Pass true to check that the users can be shared by the button instead of actually sharing them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn share_users_with_bot(source: crate::enums::KeyboardButtonSource, button_id: i32, shared_user_ids: Vec<i64>, only_check: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "shareUsersWithBot",
        "source": source,
        "button_id": button_id,
        "shared_user_ids": shared_user_ids,
        "only_check": only_check,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Shares a chat after pressing a keyboardButtonTypeRequestChat button with the bot
///
/// # Arguments
///
/// * `source` - Source of the button
/// * `button_id` - Identifier of the button
/// * `shared_chat_id` - Identifier of the shared chat
/// * `only_check` - Pass true to check that the chat can be shared by the button instead of actually sharing it. Doesn't check bot_is_member and bot_administrator_rights restrictions.
/// If the bot must be a member, then all chats from getGroupsInCommon and all chats, where the user can add the bot, are suitable. In the latter case the bot will be automatically added to the chat.
/// If the bot must be an administrator, then all chats, where the bot already has requested rights or can be added to administrators by the user, are suitable. In the latter case the bot will be automatically granted requested rights
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn share_chat_with_bot(source: crate::enums::KeyboardButtonSource, button_id: i32, shared_chat_id: i64, only_check: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "shareChatWithBot",
        "source": source,
        "button_id": button_id,
        "shared_chat_id": shared_chat_id,
        "only_check": only_check,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `chat_id` - Identifier of the chat where the query was sent
/// * `user_location` - Location of the user; pass null if unknown or the bot doesn't need user's location
/// * `query` - Text of the query
/// * `offset` - Offset of the first entry to return; use empty string to get the first chunk of results
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::InlineQueryResults)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_inline_query_results(bot_user_id: i64, chat_id: i64, user_location: Option<crate::types::Location>, query: String, offset: String, client_id: i32) -> Result<crate::enums::InlineQueryResults, crate::types::Error> {
    let request = json!({
        "@type": "getInlineQueryResults",
        "bot_user_id": bot_user_id,
        "chat_id": chat_id,
        "user_location": user_location,
        "query": query,
        "offset": offset,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the result of an inline query; for bots only
///
/// # Arguments
///
/// * `inline_query_id` - Identifier of the inline query
/// * `is_personal` - Pass true if results may be cached and returned only for the user who sent the query. By default, results may be returned to any user who sends the same query
/// * `button` - Button to be shown above inline query results; pass null if none
/// * `results` - The results of the query
/// * `cache_time` - Allowed time to cache the results of the query, in seconds
/// * `next_offset` - Offset for the next inline query; pass an empty string if there are no more results
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_inline_query(inline_query_id: i64, is_personal: bool, button: Option<crate::types::InlineQueryResultsButton>, results: Vec<crate::enums::InputInlineQueryResult>, cache_time: i32, next_offset: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "answerInlineQuery",
        "inline_query_id": inline_query_id,
        "is_personal": is_personal,
        "button": button,
        "results": results,
        "cache_time": cache_time,
        "next_offset": next_offset,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the most grossing Web App bots
///
/// # Arguments
///
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of bots to be returned; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundUsers)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_grossing_web_app_bots(offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundUsers, crate::types::Error> {
    let request = json!({
        "@type": "getGrossingWebAppBots",
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a Web App by its short name. Returns a 404 error if the Web App is not found
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `web_app_short_name` - Short name of the Web App
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundWebApp)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_web_app(bot_user_id: i64, web_app_short_name: String, client_id: i32) -> Result<crate::enums::FoundWebApp, crate::types::Error> {
    let request = json!({
        "@type": "searchWebApp",
        "bot_user_id": bot_user_id,
        "web_app_short_name": web_app_short_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a default placeholder for Web Apps of a bot. This is an offline method. Returns a 404 error if the placeholder isn't known
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Outline)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_web_app_placeholder(bot_user_id: i64, client_id: i32) -> Result<crate::enums::Outline, crate::types::Error> {
    let request = json!({
        "@type": "getWebAppPlaceholder",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTPS URL of a Web App to open after a link of the type internalLinkTypeWebApp is clicked
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which the link was clicked; pass 0 if none
/// * `bot_user_id` - Identifier of the target bot
/// * `web_app_short_name` - Short name of the Web App
/// * `start_parameter` - Start parameter from internalLinkTypeWebApp
/// * `allow_write_access` - Pass true if the current user allowed the bot to send them messages
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::WebAppUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_web_app_link_url(chat_id: i64, bot_user_id: i64, web_app_short_name: String, start_parameter: String, allow_write_access: bool, parameters: crate::types::WebAppOpenParameters, client_id: i32) -> Result<crate::enums::WebAppUrl, crate::types::Error> {
    let request = json!({
        "@type": "getWebAppLinkUrl",
        "chat_id": chat_id,
        "bot_user_id": bot_user_id,
        "web_app_short_name": web_app_short_name,
        "start_parameter": start_parameter,
        "allow_write_access": allow_write_access,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information needed to open the main Web App of a bot
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which the Web App is opened; pass 0 if none
/// * `bot_user_id` - Identifier of the target bot. If the bot is restricted for the current user, then show an error instead of calling the method
/// * `start_parameter` - Start parameter from internalLinkTypeMainWebApp
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::MainWebApp)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_main_web_app(chat_id: i64, bot_user_id: i64, start_parameter: String, parameters: crate::types::WebAppOpenParameters, client_id: i32) -> Result<crate::enums::MainWebApp, crate::types::Error> {
    let request = json!({
        "@type": "getMainWebApp",
        "chat_id": chat_id,
        "bot_user_id": bot_user_id,
        "start_parameter": start_parameter,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTPS URL of a Web App to open from the side menu, a keyboardButtonTypeWebApp button, or an inlineQueryResultsButtonTypeWebApp button
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot. If the bot is restricted for the current user, then show an error instead of calling the method
/// * `url` - The URL from a keyboardButtonTypeWebApp button, inlineQueryResultsButtonTypeWebApp button, or an empty string when the bot is opened from the side menu
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::WebAppUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_web_app_url(bot_user_id: i64, url: String, parameters: crate::types::WebAppOpenParameters, client_id: i32) -> Result<crate::enums::WebAppUrl, crate::types::Error> {
    let request = json!({
        "@type": "getWebAppUrl",
        "bot_user_id": bot_user_id,
        "url": url,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTPS URL of a Web App of a guard bot to open after receiving chatJoinResultGuardBotApprovalRequired
///
/// # Arguments
///
/// * `query_id` - Unique identifier of the join request as received in chatJoinResultGuardBotApprovalRequired
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::WebAppUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_guard_bot_web_app_url(query_id: i64, parameters: crate::types::WebAppOpenParameters, client_id: i32) -> Result<crate::enums::WebAppUrl, crate::types::Error> {
    let request = json!({
        "@type": "getGuardBotWebAppUrl",
        "query_id": query_id,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends data received from a keyboardButtonTypeWebApp Web App to a bot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `button_text` - Text of the keyboardButtonTypeWebApp button, which opened the Web App
/// * `data` - The data
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_web_app_data(bot_user_id: i64, button_text: String, data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendWebAppData",
        "bot_user_id": bot_user_id,
        "button_text": button_text,
        "data": data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that a Web App is being opened from the attachment menu, a botMenuButton button, an internalLinkTypeAttachmentMenuBot link, or an inlineKeyboardButtonTypeWebApp button.
/// For each bot, a confirmation alert about data sent to the bot must be shown once
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat in which the Web App is opened. The Web App can't be opened in secret chats
/// * `bot_user_id` - Identifier of the bot, providing the Web App. If the bot is restricted for the current user, then show an error instead of calling the method
/// * `url` - The URL from an inlineKeyboardButtonTypeWebApp button, a botMenuButton button, an internalLinkTypeAttachmentMenuBot link, or an empty string otherwise
/// * `topic_id` - Topic in which the message will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied in the message sent by the Web App; pass null if none
/// * `parameters` - Parameters to use to open the Web App
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::WebAppInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn open_web_app(chat_id: i64, bot_user_id: i64, url: String, topic_id: Option<crate::enums::MessageTopic>, reply_to: Option<crate::enums::InputMessageReplyTo>, parameters: crate::types::WebAppOpenParameters, client_id: i32) -> Result<crate::enums::WebAppInfo, crate::types::Error> {
    let request = json!({
        "@type": "openWebApp",
        "chat_id": chat_id,
        "bot_user_id": bot_user_id,
        "url": url,
        "topic_id": topic_id,
        "reply_to": reply_to,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that a previously opened Web App was closed
///
/// # Arguments
///
/// * `web_app_launch_id` - Identifier of Web App launch, received from openWebApp
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn close_web_app(web_app_launch_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeWebApp",
        "web_app_launch_id": web_app_launch_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the result of interaction with a Web App and sends corresponding message on behalf of the user to the chat from which the query originated; for bots only
///
/// # Arguments
///
/// * `web_app_query_id` - Identifier of the Web App query
/// * `result` - The result of the query
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::InlineMessageId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_web_app_query(web_app_query_id: String, result: crate::enums::InputInlineQueryResult, client_id: i32) -> Result<crate::enums::InlineMessageId, crate::types::Error> {
    let request = json!({
        "@type": "answerWebAppQuery",
        "web_app_query_id": web_app_query_id,
        "result": result,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether a file can be downloaded and saved locally by Web App request
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the bot, providing the Web App
/// * `file_name` - Name of the file
/// * `url` - URL of the file
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_web_app_file_download(bot_user_id: i64, file_name: String, url: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkWebAppFileDownload",
        "bot_user_id": bot_user_id,
        "file_name": file_name,
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a bot that can be added to attachment or side menu
///
/// # Arguments
///
/// * `bot_user_id` - Bot's user identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AttachmentMenuBot)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_attachment_menu_bot(bot_user_id: i64, client_id: i32) -> Result<crate::enums::AttachmentMenuBot, crate::types::Error> {
    let request = json!({
        "@type": "getAttachmentMenuBot",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds or removes a bot to attachment and side menu. Bot can be added to the menu, only if userTypeBot.can_be_added_to_attachment_menu == true
///
/// # Arguments
///
/// * `bot_user_id` - Bot's user identifier
/// * `is_added` - Pass true to add the bot to attachment menu; pass false to remove the bot from attachment menu
/// * `allow_write_access` - Pass true if the current user allowed the bot to send them messages. Ignored if is_added is false
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_bot_is_added_to_attachment_menu(bot_user_id: i64, is_added: bool, allow_write_access: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleBotIsAddedToAttachmentMenu",
        "bot_user_id": bot_user_id,
        "is_added": is_added,
        "allow_write_access": allow_write_access,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns up to 20 recently used inline bots in the order of their last usage
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Users)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recent_inline_bots(client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "getRecentInlineBots",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of bots owned by the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Users)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_owned_bots(client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "getOwnedBots",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets menu button for the given user or for all users; for bots only
///
/// # Arguments
///
/// * `user_id` - Identifier of the user or 0 to set menu button for all users
/// * `menu_button` - New menu button
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_menu_button(user_id: i64, menu_button: crate::types::BotMenuButton, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMenuButton",
        "user_id": user_id,
        "menu_button": menu_button,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns menu button set by the bot for the given user; for bots only
///
/// # Arguments
///
/// * `user_id` - Identifier of the user or 0 to get the default menu button
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BotMenuButton)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_menu_button(user_id: i64, client_id: i32) -> Result<crate::enums::BotMenuButton, crate::types::Error> {
    let request = json!({
        "@type": "getMenuButton",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether the specified bot can send messages to the user. Returns a 404 error if can't and the access can be granted by call to allowBotToSendMessages
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn can_bot_send_messages(bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "canBotSendMessages",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Allows the specified bot to send messages to the user
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn allow_bot_to_send_messages(bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "allowBotToSendMessages",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a custom request from a Web App
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the bot
/// * `method` - The method name
/// * `parameters` - JSON-serialized method parameters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CustomRequestResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_web_app_custom_request(bot_user_id: i64, method: String, parameters: String, client_id: i32) -> Result<crate::enums::CustomRequestResult, crate::types::Error> {
    let request = json!({
        "@type": "sendWebAppCustomRequest",
        "bot_user_id": bot_user_id,
        "method": method,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of media previews of a bot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot. The bot must have the main Web App
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BotMediaPreviews)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_media_previews(bot_user_id: i64, client_id: i32) -> Result<crate::enums::BotMediaPreviews, crate::types::Error> {
    let request = json!({
        "@type": "getBotMediaPreviews",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of media previews for the given language and the list of languages for which the bot has dedicated previews
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - A two-letter ISO 639-1 language code for which to get previews. If empty, then default previews are returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BotMediaPreviewInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_media_preview_info(bot_user_id: i64, language_code: String, client_id: i32) -> Result<crate::enums::BotMediaPreviewInfo, crate::types::Error> {
    let request = json!({
        "@type": "getBotMediaPreviewInfo",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a new media preview to the beginning of the list of media previews of a bot. Returns the added preview after addition is completed server-side. The total number of previews must not exceed getOption("bot_media_preview_count_max") for the given language
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - A two-letter ISO 639-1 language code for which preview is added. If empty, then the preview will be shown to all users for whose languages there are no dedicated previews.
/// If non-empty, then there must be an official language pack of the same name, which is returned by getLocalizationTargetInfo
/// * `content` - Content of the added preview
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BotMediaPreview)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_bot_media_preview(bot_user_id: i64, language_code: String, content: crate::enums::InputStoryContent, client_id: i32) -> Result<crate::enums::BotMediaPreview, crate::types::Error> {
    let request = json!({
        "@type": "addBotMediaPreview",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "content": content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Replaces media preview in the list of media previews of a bot. Returns the new preview after edit is completed server-side
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - Language code of the media preview to edit
/// * `file_id` - File identifier of the media to replace
/// * `content` - Content of the new preview
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BotMediaPreview)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_bot_media_preview(bot_user_id: i64, language_code: String, file_id: i32, content: crate::enums::InputStoryContent, client_id: i32) -> Result<crate::enums::BotMediaPreview, crate::types::Error> {
    let request = json!({
        "@type": "editBotMediaPreview",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "file_id": file_id,
        "content": content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes order of media previews in the list of media previews of a bot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - Language code of the media previews to reorder
/// * `file_ids` - File identifiers of the media in the new order
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_bot_media_previews(bot_user_id: i64, language_code: String, file_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderBotMediaPreviews",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "file_ids": file_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes media previews from the list of media previews of a bot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - Language code of the media previews to delete
/// * `file_ids` - File identifiers of the media to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_bot_media_previews(bot_user_id: i64, language_code: String, file_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteBotMediaPreviews",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "file_ids": file_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks whether a username can be set for a new bot. Use checkChatUsername to check username for other chat types
///
/// # Arguments
///
/// * `username` - Username to be checked
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CheckChatUsernameResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_bot_username(username: String, client_id: i32) -> Result<crate::enums::CheckChatUsernameResult, crate::types::Error> {
    let request = json!({
        "@type": "checkBotUsername",
        "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a bot which will be managed by another bot. Returns the created bot. May return an error with a message "BOT_CREATE_LIMIT_EXCEEDED"
/// if the user already owns the maximum allowed number of bots as per getOption("owned_bot_count_max"). An internal link "https:t.me/BotFather?start=deletebot" can be processed to handle the error
///
/// # Arguments
///
/// * `manager_bot_user_id` - Identifier of the bot that will manage the created bot
/// * `name` - Name of the bot; 1-64 characters
/// * `username` - Username of the bot. The username must end with "bot". Use checkBotUsername to find whether the name is suitable
/// * `via_link` - Pass true if the bot is created from an internalLinkTypeRequestManagedBot link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::User)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_bot(manager_bot_user_id: i64, name: String, username: String, via_link: bool, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "createBot",
        "manager_bot_user_id": manager_bot_user_id,
        "name": name,
        "username": username,
        "via_link": via_link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns token of a managed bot; for bots only
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the managed bot
/// * `revoke` - Pass true to revoke the current token and create a new one
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_managed_bot_token(bot_user_id: i64, revoke: bool, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getManagedBotToken",
        "bot_user_id": bot_user_id,
        "revoke": revoke,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns access settings of a managed bot; for bots only
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the managed bot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BotAccessSettings)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_managed_bot_access_settings(bot_user_id: i64, client_id: i32) -> Result<crate::enums::BotAccessSettings, crate::types::Error> {
    let request = json!({
        "@type": "getManagedBotAccessSettings",
        "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets access settings of a managed bot; for bots only
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the managed bot
/// * `settings` - New access settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_managed_bot_access_settings(bot_user_id: i64, settings: crate::types::BotAccessSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setManagedBotAccessSettings",
        "bot_user_id": bot_user_id,
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the name of a bot. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose languages there is no dedicated name
/// * `name` - New bot's name on the specified language; 0-64 characters; must be non-empty if language code is empty
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_name(bot_user_id: i64, language_code: String, name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotName",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the name of a bot in the given language. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code or an empty string
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_name(bot_user_id: i64, language_code: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getBotName",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes a profile photo for a bot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `photo` - Profile photo to set; pass null to delete the chat photo
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_profile_photo(bot_user_id: i64, photo: Option<crate::enums::InputChatPhoto>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotProfilePhoto",
        "bot_user_id": bot_user_id,
        "photo": photo,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes active state for a username of a bot. The editable username can be disabled only if there are other active usernames.
/// May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `username` - The username to change
/// * `is_active` - Pass true to activate the username; pass false to disable it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_bot_username_is_active(bot_user_id: i64, username: String, is_active: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleBotUsernameIsActive",
        "bot_user_id": bot_user_id,
        "username": username,
        "is_active": is_active,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes order of active usernames of a bot. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `usernames` - The new order of active usernames. All currently active usernames must be specified
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_bot_active_usernames(bot_user_id: i64, usernames: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderBotActiveUsernames",
        "bot_user_id": bot_user_id,
        "usernames": usernames,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the text shown in the chat with a bot if the chat is empty. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code. If empty, the description will be shown to all users for whose languages there is no dedicated description
/// * `description` - New bot's description on the specified language
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_info_description(bot_user_id: i64, language_code: String, description: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotInfoDescription",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "description": description,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the text shown in the chat with a bot if the chat is empty in the given language. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code or an empty string
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_info_description(bot_user_id: i64, language_code: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getBotInfoDescription",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the text shown on a bot's profile page and sent together with the link when users share the bot. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code. If empty, the short description will be shown to all users for whose languages there is no dedicated description
/// * `short_description` - New bot's short description on the specified language
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_info_short_description(bot_user_id: i64, language_code: String, short_description: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotInfoShortDescription",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "short_description": short_description,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the text shown on a bot's profile page and sent together with the link when users share the bot in the given language. Can be called only if userTypeBot.can_be_edited == true
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code or an empty string
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_info_short_description(bot_user_id: i64, language_code: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getBotInfoShortDescription",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the verification status of a user or a chat by an owned bot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the owned bot, which will verify the user or the chat
/// * `verified_id` - Identifier of the user or the supergroup or channel chat, which will be verified by the bot
/// * `custom_description` - Custom description of verification reason; 0-getOption("bot_verification_custom_description_length_max").
/// If empty, then "was verified by organization "organization_name"" will be used as description. Can be specified only if the bot is allowed to provide custom description
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_message_sender_bot_verification(bot_user_id: i64, verified_id: crate::enums::MessageSender, custom_description: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMessageSenderBotVerification",
        "bot_user_id": bot_user_id,
        "verified_id": verified_id,
        "custom_description": custom_description,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes the verification status of a user or a chat by an owned bot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the owned bot, which verified the user or the chat
/// * `verified_id` - Identifier of the user or the supergroup or channel chat, which verification is removed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_message_sender_bot_verification(bot_user_id: i64, verified_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeMessageSenderBotVerification",
        "bot_user_id": bot_user_id,
        "verified_id": verified_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only
///
/// # Arguments
///
/// * `pending_update_count` - The number of pending updates
/// * `error_message` - The last error message
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_updates_status(pending_update_count: i32, error_message: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotUpdatesStatus",
        "pending_update_count": pending_update_count,
        "error_message": error_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

