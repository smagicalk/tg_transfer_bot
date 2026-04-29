#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of emoji statuses, which can't be used as chat emoji status, even if they are from a sticker set with is_allowed_as_chat_emoji_status == true
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_disallowed_chat_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatusCustomEmojis, crate::types::Error> {
    let request = json!({
        "@type": "getDisallowedChatEmojiStatuses",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
