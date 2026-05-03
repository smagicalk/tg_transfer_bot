use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Cancels reset of 2-step verification password. The method can be called if passwordState.pending_reset_date > 0
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn cancel_password_reset(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "cancelPasswordReset",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
