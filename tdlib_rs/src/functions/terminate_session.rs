use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Terminates a session of the current user
/// # Arguments
/// * `session_id` - Session identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn terminate_session(session_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "terminateSession",
    "session_id": session_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
