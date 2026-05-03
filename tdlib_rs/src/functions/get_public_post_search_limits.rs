use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks public post search limits without actually performing the search
/// # Arguments
/// * `query` - Query that will be searched for
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_public_post_search_limits(
    query: String,
    client_id: i32,
) -> Result<crate::enums::PublicPostSearchLimits, crate::types::Error> {
    let request = json!({
    "@type": "getPublicPostSearchLimits",
    "query": query,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
