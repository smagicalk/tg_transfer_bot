use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of custom emoji, which can be used as forum topic icon by all users
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_forum_topic_default_icons(
    client_id: i32,
) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
    "@type": "getForumTopicDefaultIcons",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
