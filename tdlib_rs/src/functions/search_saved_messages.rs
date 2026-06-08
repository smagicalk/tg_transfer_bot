use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for messages tagged by the given reaction and with the given words in the Saved Messages chat; for Telegram Premium users only.
/// Returns the results in reverse chronological order, i.e. in order of decreasing message_id.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// # Arguments
/// * `saved_messages_topic_id` - If not 0, only messages in the specified Saved Messages topic will be considered; pass 0 to consider all messages
/// * `tag` - Tag to search for; pass null to return all suitable messages
/// * `query` - Query to search for
/// * `from_message_id` - Identifier of the message starting from which messages must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number to get the specified message and some newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than -offset.
/// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_saved_messages(
    saved_messages_topic_id: i64,
    tag: Option<crate::enums::ReactionType>,
    query: String,
    from_message_id: i64,
    offset: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::FoundChatMessages, crate::types::Error> {
    let request = json!({
    "@type": "searchSavedMessages",
    "saved_messages_topic_id": saved_messages_topic_id,
    "tag": tag,
    "query": query,
    "from_message_id": from_message_id,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
