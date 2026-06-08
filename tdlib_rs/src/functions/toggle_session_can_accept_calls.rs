use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether a session can accept incoming calls
/// # Arguments
/// * `session_id` - Session identifier
/// * `can_accept_calls` - Pass true to allow accepting incoming calls by the session; pass false otherwise
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_session_can_accept_calls(
    session_id: i64,
    can_accept_calls: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleSessionCanAcceptCalls",
    "session_id": session_id,
    "can_accept_calls": can_accept_calls,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
