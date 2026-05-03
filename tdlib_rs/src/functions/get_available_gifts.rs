use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns gifts that can be sent to other users and channel chats
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_available_gifts(
    client_id: i32,
) -> Result<crate::enums::AvailableGifts, crate::types::Error> {
    let request = json!({
    "@type": "getAvailableGifts",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
