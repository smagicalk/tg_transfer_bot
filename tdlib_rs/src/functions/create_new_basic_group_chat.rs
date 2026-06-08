use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns information about the newly created chat
/// # Arguments
/// * `user_ids` - Identifiers of users to be added to the basic group; may be empty to create a basic group without other members
/// * `title` - Title of the new basic group; 1-128 characters
/// * `message_auto_delete_time` - Message auto-delete time value, in seconds; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_new_basic_group_chat(
    user_ids: Vec<i64>,
    title: String,
    message_auto_delete_time: i32,
    client_id: i32,
) -> Result<crate::enums::CreatedBasicGroupChat, crate::types::Error> {
    let request = json!({
    "@type": "createNewBasicGroupChat",
    "user_ids": user_ids,
    "title": title,
    "message_auto_delete_time": message_auto_delete_time,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
