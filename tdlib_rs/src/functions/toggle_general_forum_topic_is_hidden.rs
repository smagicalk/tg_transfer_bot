use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether a General topic is hidden in a forum supergroup chat; requires can_manage_topics administrator right in the supergroup
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `is_hidden` - Pass true to hide and close the General topic; pass false to unhide it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_general_forum_topic_is_hidden(
    chat_id: i64,
    is_hidden: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleGeneralForumTopicIsHidden",
    "chat_id": chat_id,
    "is_hidden": is_hidden,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
