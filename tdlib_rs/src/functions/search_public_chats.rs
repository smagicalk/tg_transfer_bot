#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches public chats by looking for specified query in their username and title. Currently, only private chats, supergroups and channels can be public. Returns a meaningful number of results.
/// Excludes private chats with contacts and chats from the chat list from the results
/// # Arguments
/// * `query` - Query to search for
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_public_chats(query: String, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicChats",
        "query": query,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
