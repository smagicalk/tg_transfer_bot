#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the emoji status of a chat. Use chatBoostLevelFeatures.can_set_emoji_status to check whether an emoji status can be set. Requires can_change_info administrator right
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `emoji_status` - New emoji status; pass null to remove emoji status
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_emoji_status(chat_id: i64, emoji_status: Option<crate::types::EmojiStatus>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatEmojiStatus",
        "chat_id": chat_id,
        "emoji_status": emoji_status,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
