use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i.e., in order of decreasing event_id)
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `query` - Search query by which to filter events
/// * `from_event_id` - Identifier of an event from which to return results. Use 0 to get results from the latest events
/// * `limit` - The maximum number of events to return; up to 100
/// * `filters` - The types of events to return; pass null to get chat events of all types
/// * `user_ids` - User identifiers by which to filter events. By default, events relating to all users will be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_event_log(
    chat_id: i64,
    query: String,
    from_event_id: i64,
    limit: i32,
    filters: Option<crate::types::ChatEventLogFilters>,
    user_ids: Vec<i64>,
    client_id: i32,
) -> Result<crate::enums::ChatEvents, crate::types::Error> {
    let request = json!({
    "@type": "getChatEventLog",
    "chat_id": chat_id,
    "query": query,
    "from_event_id": from_event_id,
    "limit": limit,
    "filters": filters,
    "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
