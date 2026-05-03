use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Activates stealth mode for stories, which hides all views of stories from the current user in the last "story_stealth_mode_past_period" seconds
/// and for the next "story_stealth_mode_future_period" seconds; for Telegram Premium users only
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn activate_story_stealth_mode(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "activateStoryStealthMode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
