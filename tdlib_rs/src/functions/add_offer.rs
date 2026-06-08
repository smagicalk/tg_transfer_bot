use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a suggested post based on a previously sent message in a channel direct messages chat. Can be also used to suggest price or time change for an existing suggested post.
/// Returns the sent message
/// # Arguments
/// * `chat_id` - Identifier of the channel direct messages chat
/// * `message_id` - Identifier of the message in the chat which will be sent as suggested post. Use messageProperties.can_add_offer to check whether an offer can be added
/// or messageProperties.can_edit_suggested_post_info to check whether price or time of sending of the post can be changed
/// * `options` - Options to be used to send the message. New information about the suggested post must always be specified
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_offer(
    chat_id: i64,
    message_id: i64,
    options: crate::types::MessageSendOptions,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "addOffer",
    "chat_id": chat_id,
    "message_id": message_id,
    "options": options,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
