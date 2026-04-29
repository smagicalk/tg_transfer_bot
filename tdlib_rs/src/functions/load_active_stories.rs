#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Loads more active stories from a story list. The loaded stories will be sent through updates. Active stories are sorted by
/// the pair (active_stories.order, active_stories.story_poster_chat_id) in descending order. Returns a 404 error if all active stories have been loaded
/// # Arguments
/// * `story_list` - The story list in which to load active stories
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn load_active_stories(story_list: crate::enums::StoryList, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadActiveStories",
        "story_list": story_list,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
