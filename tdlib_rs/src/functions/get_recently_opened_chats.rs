use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns recently opened chats. This is an offline method. Returns chats in the order of last opening
/// # Arguments
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recently_opened_chats(
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getRecentlyOpenedChats",
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
