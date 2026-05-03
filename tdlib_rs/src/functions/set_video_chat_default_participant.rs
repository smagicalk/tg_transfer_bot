use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes default participant identifier, on whose behalf a video chat in the chat will be joined
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `default_participant_id` - Default group call participant identifier to join the video chats in the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_video_chat_default_participant(
    chat_id: i64,
    default_participant_id: crate::enums::MessageSender,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setVideoChatDefaultParticipant",
    "chat_id": chat_id,
    "default_participant_id": default_participant_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
