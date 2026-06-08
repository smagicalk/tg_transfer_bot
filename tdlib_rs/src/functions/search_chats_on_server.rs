use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the main chat list
/// # Arguments
/// * `query` - Query to search for
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_chats_on_server(
    query: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "searchChatsOnServer",
    "query": query,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
