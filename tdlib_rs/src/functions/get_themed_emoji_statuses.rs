use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns up to 8 emoji statuses, which must be shown right after the default Premium Badge in the emoji status list for self status
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_themed_emoji_statuses(
    client_id: i32,
) -> Result<crate::enums::EmojiStatusCustomEmojis, crate::types::Error> {
    let request = json!({
    "@type": "getThemedEmojiStatuses",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
