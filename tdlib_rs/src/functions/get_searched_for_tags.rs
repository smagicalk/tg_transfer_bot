#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns recently searched for hashtags or cashtags by their prefix
/// # Arguments
/// * `tag_prefix` - Prefix of hashtags or cashtags to return
/// * `limit` - The maximum number of items to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_searched_for_tags(tag_prefix: String, limit: i32, client_id: i32) -> Result<crate::enums::Hashtags, crate::types::Error> {
    let request = json!({
        "@type": "getSearchedForTags",
        "tag_prefix": tag_prefix,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
