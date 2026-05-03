use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets title of a video chat; requires groupCall.can_be_managed right
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `title` - New group call title; 1-64 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_video_chat_title(
    group_call_id: i32,
    title: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setVideoChatTitle",
    "group_call_id": group_call_id,
    "title": title,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
