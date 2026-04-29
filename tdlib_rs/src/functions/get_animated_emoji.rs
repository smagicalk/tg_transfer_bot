#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an animated emoji corresponding to a given emoji. Returns a 404 error if the emoji has no animated emoji
/// # Arguments
/// * `emoji` - The emoji
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_animated_emoji(emoji: String, client_id: i32) -> Result<crate::enums::AnimatedEmoji, crate::types::Error> {
    let request = json!({
        "@type": "getAnimatedEmoji",
        "emoji": emoji,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
