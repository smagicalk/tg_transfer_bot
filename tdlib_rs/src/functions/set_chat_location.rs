use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `location` - New location for the chat; must be valid and not null
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_location(
    chat_id: i64,
    location: crate::types::ChatLocation,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setChatLocation",
    "chat_id": chat_id,
    "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
