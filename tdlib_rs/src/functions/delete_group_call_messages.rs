use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes messages in a group call; for live story calls only. Requires groupCallMessage.can_be_deleted right
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `message_ids` - Identifiers of the messages to be deleted
/// * `report_spam` - Pass true to report the messages as spam
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_group_call_messages(
    group_call_id: i32,
    message_ids: Vec<i32>,
    report_spam: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteGroupCallMessages",
    "group_call_id": group_call_id,
    "message_ids": message_ids,
    "report_spam": report_spam,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
