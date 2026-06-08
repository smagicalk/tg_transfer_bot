use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Closes a secret chat, effectively transferring its state to secretChatStateClosed
/// # Arguments
/// * `secret_chat_id` - Secret chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn close_secret_chat(
    secret_chat_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "closeSecretChat",
    "secret_chat_id": secret_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
