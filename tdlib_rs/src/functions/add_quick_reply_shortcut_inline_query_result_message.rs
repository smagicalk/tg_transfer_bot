use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a message to a quick reply shortcut via inline bot. If shortcut doesn't exist and there are less than getOption("quick_reply_shortcut_count_max") shortcuts, then a new shortcut is created.
/// The shortcut must not contain more than getOption("quick_reply_shortcut_message_count_max") messages after adding the new message. Returns the added message
/// # Arguments
/// * `shortcut_name` - Name of the target shortcut
/// * `reply_to_message_id` - Identifier of a quick reply message in the same shortcut to be replied; pass 0 if none
/// * `query_id` - Identifier of the inline query
/// * `result_id` - Identifier of the inline query result
/// * `hide_via_bot` - Pass true to hide the bot, via which the message is sent. Can be used only for bots getOption("animation_search_bot_username"), getOption("photo_search_bot_username"), and getOption("venue_search_bot_username")
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_quick_reply_shortcut_inline_query_result_message(
    shortcut_name: String,
    reply_to_message_id: i64,
    query_id: i64,
    result_id: String,
    hide_via_bot: bool,
    client_id: i32,
) -> Result<crate::enums::QuickReplyMessage, crate::types::Error> {
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
