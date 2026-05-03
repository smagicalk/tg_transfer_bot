use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)).
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// # Arguments
/// * `chat_list` - Chat list in which to search messages; pass null to search in all chats regardless of their chat list. Only Main and Archive chat lists are supported
/// * `query` - Query to search for
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `filter` - Additional filter for messages to search; pass null to search for all messages. Filters searchMessagesFilterMention, searchMessagesFilterUnreadMention, searchMessagesFilterUnreadReaction, searchMessagesFilterFailedToSend, and searchMessagesFilterPinned are unsupported in this function
/// * `chat_type_filter` - Additional filter for type of the chat of the searched messages; pass null to search for messages in all chats
/// * `min_date` - If not 0, the minimum date of the messages to return
/// * `max_date` - If not 0, the maximum date of the messages to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_messages(
    chat_list: Option<crate::enums::ChatList>,
    query: String,
    offset: String,
    limit: i32,
    filter: Option<crate::enums::SearchMessagesFilter>,
    chat_type_filter: Option<crate::enums::SearchMessagesChatTypeFilter>,
    min_date: i32,
    max_date: i32,
    client_id: i32,
) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
    "@type": "searchMessages",
    "chat_list": chat_list,
    "query": query,
    "offset": offset,
    "limit": limit,
    "filter": filter,
    "chat_type_filter": chat_type_filter,
    "min_date": min_date,
    "max_date": max_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
