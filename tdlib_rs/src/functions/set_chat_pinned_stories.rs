#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the list of pinned stories on a chat page; requires can_edit_stories administrator right in the chat
/// # Arguments
/// * `chat_id` - Identifier of the chat that posted the stories
/// * `story_ids` - New list of pinned stories. All stories must be posted to the chat page first. There can be up to getOption("pinned_story_count_max") pinned stories on a chat page
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_pinned_stories(chat_id: i64, story_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatPinnedStories",
        "chat_id": chat_id,
        "story_ids": story_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
