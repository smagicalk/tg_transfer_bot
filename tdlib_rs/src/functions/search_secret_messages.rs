use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance, the number of returned messages is chosen by TDLib
/// # Arguments
/// * `chat_id` - Identifier of the chat in which to search. Specify 0 to search in all secret chats
/// * `query` - Query to search for. If empty, searchChatMessages must be used instead
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `filter` - Additional filter for messages to search; pass null to search for all messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_secret_messages(
    chat_id: i64,
    query: String,
    offset: String,
    limit: i32,
    filter: Option<crate::enums::SearchMessagesFilter>,
    client_id: i32,
) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
    "@type": "searchSecretMessages",
    "chat_id": chat_id,
    "query": query,
    "offset": offset,
    "limit": limit,
    "filter": filter,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
