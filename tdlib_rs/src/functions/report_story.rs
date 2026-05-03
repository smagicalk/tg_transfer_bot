use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Reports a story to the Telegram moderators
/// # Arguments
/// * `story_poster_chat_id` - The identifier of the poster of the story to report
/// * `story_id` - The identifier of the story to report
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `text` - Additional report details; 0-1024 characters; leave empty for the initial request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_story(
    story_poster_chat_id: i64,
    story_id: i32,
    option_id: String,
    text: String,
    client_id: i32,
) -> Result<crate::enums::ReportStoryResult, crate::types::Error> {
    let request = json!({
    "@type": "reportStory",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    "option_id": option_id,
    "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
