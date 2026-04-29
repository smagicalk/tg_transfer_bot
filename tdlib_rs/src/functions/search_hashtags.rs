#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for recently used hashtags by their prefix
/// # Arguments
/// * `prefix` - Hashtag prefix to search for
/// * `limit` - The maximum number of hashtags to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_hashtags(prefix: String, limit: i32, client_id: i32) -> Result<crate::enums::Hashtags, crate::types::Error> {
    let request = json!({
        "@type": "searchHashtags",
        "prefix": prefix,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
