#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Starts recording of an active group call; for video chats only. Requires groupCall.can_be_managed right
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `title` - Group call recording title; 0-64 characters
/// * `record_video` - Pass true to record a video file instead of an audio file
/// * `use_portrait_orientation` - Pass true to use portrait orientation for video instead of landscape one
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn start_group_call_recording(group_call_id: i32, title: String, record_video: bool, use_portrait_orientation: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "startGroupCallRecording",
        "group_call_id": group_call_id,
        "title": title,
        "record_video": record_video,
        "use_portrait_orientation": use_portrait_orientation,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
