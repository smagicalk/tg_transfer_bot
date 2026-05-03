use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks whether the current user can post a story on behalf of a chat; requires can_post_stories administrator right for supergroup and channel chats
/// # Arguments
/// * `chat_id` - Chat identifier. Pass Saved Messages chat identifier when posting a story on behalf of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn can_post_story(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::CanPostStoryResult, crate::types::Error> {
    let request = json!({
    "@type": "canPostStory",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
