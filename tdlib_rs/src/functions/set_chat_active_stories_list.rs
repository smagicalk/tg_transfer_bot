#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes story list in which stories from the chat are shown
/// # Arguments
/// * `chat_id` - Identifier of the chat that posted stories
/// * `story_list` - New list for active stories posted by the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_active_stories_list(chat_id: i64, story_list: crate::enums::StoryList, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatActiveStoriesList",
        "chat_id": chat_id,
        "story_list": story_list,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
