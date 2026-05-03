use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn test_network(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "testNetwork",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
