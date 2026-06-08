use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a new secret chat. Returns the newly created chat
/// # Arguments
/// * `user_id` - Identifier of the target user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_new_secret_chat(
    user_id: i64,
    client_id: i32,
) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
    "@type": "createNewSecretChat",
    "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
