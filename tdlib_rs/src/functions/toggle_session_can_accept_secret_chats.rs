#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether a session can accept incoming secret chats
/// # Arguments
/// * `session_id` - Session identifier
/// * `can_accept_secret_chats` - Pass true to allow accepting secret chats by the session; pass false otherwise
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_session_can_accept_secret_chats(session_id: i64, can_accept_secret_chats: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSessionCanAcceptSecretChats",
        "session_id": session_id,
        "can_accept_secret_chats": can_accept_secret_chats,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
