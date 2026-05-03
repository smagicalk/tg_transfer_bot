use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns all website where the current user used Telegram to log in
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_connected_websites(
    client_id: i32,
) -> Result<crate::enums::ConnectedWebsites, crate::types::Error> {
    let request = json!({
    "@type": "getConnectedWebsites",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
