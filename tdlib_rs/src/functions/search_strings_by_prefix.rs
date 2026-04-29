#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches specified query by word prefixes in the provided strings. Returns 0-based positions of strings that matched. Can be called synchronously
/// # Arguments
/// * `strings` - The strings to search in for the query
/// * `query` - Query to search for
/// * `limit` - The maximum number of objects to return
/// * `return_none_for_empty_query` - Pass true to receive no results for an empty query
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_strings_by_prefix(strings: Vec<String>, query: String, limit: i32, return_none_for_empty_query: bool, client_id: i32) -> Result<crate::enums::FoundPositions, crate::types::Error> {
    let request = json!({
        "@type": "searchStringsByPrefix",
        "strings": strings,
        "query": query,
        "limit": limit,
        "return_none_for_empty_query": return_none_for_empty_query,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
