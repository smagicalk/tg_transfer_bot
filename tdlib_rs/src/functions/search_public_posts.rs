#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for public channel posts using the given query. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// # Arguments
/// * `query` - Query to search for
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `star_count` - The Telegram Star amount the user agreed to pay for the search; pass 0 for free searches
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_public_posts(query: String, offset: String, limit: i32, star_count: i64, client_id: i32) -> Result<crate::enums::FoundPublicPosts, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicPosts",
        "query": query,
        "offset": offset,
        "limit": limit,
        "star_count": star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
