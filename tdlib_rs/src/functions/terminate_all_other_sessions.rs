use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Terminates all other sessions of the current user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn terminate_all_other_sessions(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "terminateAllOtherSessions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
