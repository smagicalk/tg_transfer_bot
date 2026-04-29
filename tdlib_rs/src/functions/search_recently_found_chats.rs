#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for the specified query in the title and username of up to 50 recently found chats. This is an offline method
/// # Arguments
/// * `query` - Query to search for
/// * `limit` - The maximum number of chats to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_recently_found_chats(query: String, limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "searchRecentlyFoundChats",
        "query": query,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
