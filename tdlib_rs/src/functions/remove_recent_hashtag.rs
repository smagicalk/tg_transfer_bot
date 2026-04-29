#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes a hashtag from the list of recently used hashtags
/// # Arguments
/// * `hashtag` - Hashtag to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_recent_hashtag(hashtag: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeRecentHashtag",
        "hashtag": hashtag,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
