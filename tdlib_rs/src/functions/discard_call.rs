use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Discards a call
/// # Arguments
/// * `call_id` - Call identifier
/// * `is_disconnected` - Pass true if the user was disconnected
/// * `invite_link` - If the call was upgraded to a group call, pass invite link to the group call
/// * `duration` - The call duration, in seconds
/// * `is_video` - Pass true if the call was a video call
/// * `connection_id` - Identifier of the connection used during the call
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn discard_call(
    call_id: i32,
    is_disconnected: bool,
    invite_link: String,
    duration: i32,
    is_video: bool,
    connection_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "discardCall",
    "call_id": call_id,
    "is_disconnected": is_disconnected,
    "invite_link": invite_link,
    "duration": duration,
    "is_video": is_video,
    "connection_id": connection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
