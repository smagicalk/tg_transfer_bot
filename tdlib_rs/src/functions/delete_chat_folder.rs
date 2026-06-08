use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes existing chat folder
/// # Arguments
/// * `chat_folder_id` - Chat folder identifier
/// * `leave_chat_ids` - Identifiers of the chats to leave. The chats must be pinned or always included in the folder
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_folder(
    chat_folder_id: i32,
    leave_chat_ids: Vec<i64>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteChatFolder",
    "chat_folder_id": chat_folder_id,
    "leave_chat_ids": leave_chat_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
