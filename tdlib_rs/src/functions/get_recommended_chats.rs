use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of channel chats recommended to the current user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recommended_chats(
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getRecommendedChats",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
