use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the emoji status of a user; for bots only
/// # Arguments
/// * `user_id` - Identifier of the user
/// * `emoji_status` - New emoji status; pass null to switch to the default badge
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_user_emoji_status(
    user_id: i64,
    emoji_status: Option<crate::types::EmojiStatus>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setUserEmojiStatus",
    "user_id": user_id,
    "emoji_status": emoji_status,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
