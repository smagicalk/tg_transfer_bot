use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of recently used stickers
/// # Arguments
/// * `is_attached` - Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recent_stickers(
    is_attached: bool,
    client_id: i32,
) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
    "@type": "getRecentStickers",
    "is_attached": is_attached,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
