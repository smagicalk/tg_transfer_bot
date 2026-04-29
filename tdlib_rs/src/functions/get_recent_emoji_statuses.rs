#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns recent emoji statuses for self status
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recent_emoji_statuses(client_id: i32) -> Result<crate::enums::EmojiStatuses, crate::types::Error> {
    let request = json!({
        "@type": "getRecentEmojiStatuses",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
