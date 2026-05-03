use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a hashtag or a cashtag from the list of recently searched for hashtags or cashtags
/// # Arguments
/// * `tag` - Hashtag or cashtag to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_searched_for_tag(
    tag: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeSearchedForTag",
    "tag": tag,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
