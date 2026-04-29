#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the emoji status of the current user; for Telegram Premium users only
/// # Arguments
/// * `emoji_status` - New emoji status; pass null to switch to the default badge
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_emoji_status(emoji_status: Option<crate::types::EmojiStatus>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setEmojiStatus",
        "emoji_status": emoji_status,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
