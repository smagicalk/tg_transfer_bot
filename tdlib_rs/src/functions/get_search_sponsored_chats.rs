#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns sponsored chats to be shown in the search results
/// # Arguments
/// * `query` - Query the user searches for
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_search_sponsored_chats(query: String, client_id: i32) -> Result<crate::enums::SponsoredChats, crate::types::Error> {
    let request = json!({
        "@type": "getSearchSponsoredChats",
        "query": query,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
