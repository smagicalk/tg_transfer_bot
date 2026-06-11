use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes a story posted by the bot on behalf of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `story_id` - Identifier of the story to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_business_story(
    business_connection_id: String,
    story_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteBusinessStory",
    "business_connection_id": business_connection_id,
    "story_id": story_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
