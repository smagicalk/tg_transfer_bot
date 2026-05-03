use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an existing chat corresponding to a given user
/// # Arguments
/// * `user_id` - User identifier
/// * `force` - Pass true to create the chat without a network request. In this case all information about the chat except its type, title and photo can be incorrect
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_private_chat(
    user_id: i64,
    force: bool,
    client_id: i32,
) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
    "@type": "createPrivateChat",
    "user_id": user_id,
    "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
