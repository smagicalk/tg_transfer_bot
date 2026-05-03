use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes messages
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_ids` - Identifiers of the messages to be deleted. Use messageProperties.can_be_deleted_only_for_self and messageProperties.can_be_deleted_for_all_users to get suitable messages
/// * `revoke` - Pass true to delete messages for all chat members. Always true for supergroups, channels and secret chats
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_messages(
    chat_id: i64,
    message_ids: Vec<i64>,
    revoke: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteMessages",
    "chat_id": chat_id,
    "message_ids": message_ids,
    "revoke": revoke,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
