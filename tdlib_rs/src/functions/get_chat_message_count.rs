use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns approximate number of messages of the specified type in the chat or its topic
/// # Arguments
/// * `chat_id` - Identifier of the chat in which to count messages
/// * `topic_id` - Pass topic identifier to get number of messages only in specific topic; pass null to get number of messages in all topics; message threads aren't supported
/// * `filter` - Filter for message content; searchMessagesFilterEmpty is unsupported in this function
/// * `return_local` - Pass true to get the number of messages without sending network requests, or -1 if the number of messages is unknown locally
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_count(
    chat_id: i64,
    topic_id: Option<crate::enums::MessageTopic>,
    filter: crate::enums::SearchMessagesFilter,
    return_local: bool,
    client_id: i32,
) -> Result<crate::enums::Count, crate::types::Error> {
    let request = json!({
    "@type": "getChatMessageCount",
    "chat_id": chat_id,
    "topic_id": topic_id,
    "filter": filter,
    "return_local": return_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
