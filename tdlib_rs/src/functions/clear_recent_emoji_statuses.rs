#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Clears the list of recently used emoji statuses for self status
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn clear_recent_emoji_statuses(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearRecentEmojiStatuses",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
