use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Blocks an original sender of a message in the Replies chat
/// # Arguments
/// * `message_id` - The identifier of an incoming message in the Replies chat
/// * `delete_message` - Pass true to delete the message
/// * `delete_all_messages` - Pass true to delete all messages from the same sender
/// * `report_spam` - Pass true to report the sender to the Telegram moderators
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn block_message_sender_from_replies(
    message_id: i64,
    delete_message: bool,
    delete_all_messages: bool,
    report_spam: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "blockMessageSenderFromReplies",
    "message_id": message_id,
    "delete_message": delete_message,
    "delete_all_messages": delete_all_messages,
    "report_spam": report_spam,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
