use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
/// # Arguments
/// * `title` - Title of the new chat; 1-128 characters
/// * `is_forum` - Pass true to create a forum supergroup chat
/// * `is_channel` - Pass true to create a channel chat; ignored if a forum is created
/// * `description` - Chat description; 0-255 characters
/// * `location` - Chat location if a location-based supergroup is being created; pass null to create an ordinary supergroup chat
/// * `message_auto_delete_time` - Message auto-delete time value, in seconds; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `for_import` - Pass true to create a supergroup for importing messages using importMessages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_new_supergroup_chat(
    title: String,
    is_forum: bool,
    is_channel: bool,
    description: String,
    location: Option<crate::types::ChatLocation>,
    message_auto_delete_time: i32,
    for_import: bool,
    client_id: i32,
) -> Result<crate::enums::Chat, crate::types::Error> {
    let request = json!({
    "@type": "createNewSupergroupChat",
    "title": title,
    "is_forum": is_forum,
    "is_channel": is_channel,
    "description": description,
    "location": location,
    "message_auto_delete_time": message_auto_delete_time,
    "for_import": for_import,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
