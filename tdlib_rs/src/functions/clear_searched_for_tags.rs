use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Clears the list of recently searched for hashtags or cashtags
/// # Arguments
/// * `clear_cashtags` - Pass true to clear the list of recently searched for cashtags; otherwise, the list of recently searched for hashtags will be cleared
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn clear_searched_for_tags(
    clear_cashtags: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "clearSearchedForTags",
    "clear_cashtags": clear_cashtags,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
