use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns up to 20 recently used inline bots in the order of their last usage
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recent_inline_bots(
    client_id: i32,
) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
    "@type": "getRecentInlineBots",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
