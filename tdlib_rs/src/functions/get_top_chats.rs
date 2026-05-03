use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of frequently used chats
/// # Arguments
/// * `category` - Category of chats to be returned
/// * `limit` - The maximum number of chats to be returned; up to 30
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_top_chats(
    category: crate::enums::TopChatCategory,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getTopChats",
    "category": category,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
