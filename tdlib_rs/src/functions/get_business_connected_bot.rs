use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the business bot that is connected to the current user account. Returns a 404 error if there is no connected bot
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_business_connected_bot(
    client_id: i32,
) -> Result<crate::enums::BusinessConnectedBot, crate::types::Error> {
    let request = json!({
    "@type": "getBusinessConnectedBot",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
