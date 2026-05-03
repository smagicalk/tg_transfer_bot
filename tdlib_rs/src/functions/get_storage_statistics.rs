use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns storage usage statistics. Can be called before authorization
/// # Arguments
/// * `chat_limit` - The maximum number of chats with the largest storage usage for which separate statistics need to be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_storage_statistics(
    chat_limit: i32,
    client_id: i32,
) -> Result<crate::enums::StorageStatistics, crate::types::Error> {
    let request = json!({
    "@type": "getStorageStatistics",
    "chat_limit": chat_limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
