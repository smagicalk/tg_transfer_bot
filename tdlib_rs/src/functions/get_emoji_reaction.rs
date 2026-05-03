use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about an emoji reaction. Returns a 404 error if the reaction is not found
/// # Arguments
/// * `emoji` - Text representation of the reaction
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_emoji_reaction(
    emoji: String,
    client_id: i32,
) -> Result<crate::enums::EmojiReaction, crate::types::Error> {
    let request = json!({
    "@type": "getEmojiReaction",
    "emoji": emoji,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
