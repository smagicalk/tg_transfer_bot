use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message
/// # Arguments
/// * `chat_id` - Target chat
/// * `topic_id` - Topic in which the message will be sent; pass null if none
/// * `reply_to` - Information about the message or story to be replied; pass null if none
/// * `options` - Options to be used to send the message; pass null to use default options
/// * `query_id` - Identifier of the inline query
/// * `result_id` - Identifier of the inline query result
/// * `hide_via_bot` - Pass true to hide the bot, via which the message is sent. Can be used only for bots getOption("animation_search_bot_username"), getOption("photo_search_bot_username"), and getOption("venue_search_bot_username")
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_inline_query_result_message(
    chat_id: i64,
    topic_id: Option<crate::enums::MessageTopic>,
    reply_to: Option<crate::enums::InputMessageReplyTo>,
    options: Option<crate::types::MessageSendOptions>,
    query_id: i64,
    result_id: String,
    hide_via_bot: bool,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
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
