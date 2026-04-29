#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates a video chat (a group call bound to a chat); for basic groups, supergroups and channels only; requires can_manage_video_chats administrator right
/// # Arguments
/// * `chat_id` - Identifier of a chat in which the video chat will be created
/// * `title` - Group call title; if empty, chat title will be used
/// * `start_date` - Point in time (Unix timestamp) when the group call is expected to be started by an administrator; 0 to start the video chat immediately. The date must be at least 10 seconds and at most 8 days in the future
/// * `is_rtmp_stream` - Pass true to create an RTMP stream instead of an ordinary video chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_video_chat(chat_id: i64, title: String, start_date: i32, is_rtmp_stream: bool, client_id: i32) -> Result<crate::enums::GroupCallId, crate::types::Error> {
    let request = json!({
        "@type": "createVideoChat",
        "chat_id": chat_id,
        "title": title,
        "start_date": start_date,
        "is_rtmp_stream": is_rtmp_stream,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
