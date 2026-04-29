#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Clears the list of recently used stickers
/// # Arguments
/// * `is_attached` - Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn clear_recent_stickers(is_attached: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearRecentStickers",
        "is_attached": is_attached,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
