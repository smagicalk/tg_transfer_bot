#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes all messages sent by the specified message sender in a group call; for live story calls only. Requires groupCall.can_delete_messages right
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `sender_id` - Identifier of the sender of messages to delete
/// * `report_spam` - Pass true to report the messages as spam
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_group_call_messages_by_sender(group_call_id: i32, sender_id: crate::enums::MessageSender, report_spam: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteGroupCallMessagesBySender",
        "group_call_id": group_call_id,
        "sender_id": sender_id,
        "report_spam": report_spam,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
