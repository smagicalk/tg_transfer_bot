use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Clears the list of recently used reactions
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn clear_recent_reactions(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "clearRecentReactions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
