use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Declines an invitation to an active group call via messageGroupCall. Can be called both by the sender and the receiver of the invitation
/// # Arguments
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message of the type messageGroupCall
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn decline_group_call_invitation(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "declineGroupCallInvitation",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
