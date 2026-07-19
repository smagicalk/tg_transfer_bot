use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only
/// # Arguments
/// * `pending_update_count` - The number of pending updates
/// * `error_message` - The last error message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_updates_status(
    pending_update_count: i32,
    error_message: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBotUpdatesStatus",
    "pending_update_count": pending_update_count,
    "error_message": error_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
