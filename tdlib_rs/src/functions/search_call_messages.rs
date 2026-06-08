use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for call and group call messages. Returns the results in reverse chronological order (i.e., in order of decreasing message_id). For optimal performance, the number of returned messages is chosen by TDLib
/// # Arguments
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `only_missed` - Pass true to search only for messages with missed/declined calls
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_call_messages(
    offset: String,
    limit: i32,
    only_missed: bool,
    client_id: i32,
) -> Result<crate::enums::FoundMessages, crate::types::Error> {
    let request = json!({
    "@type": "searchCallMessages",
    "offset": offset,
    "limit": limit,
    "only_missed": only_missed,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
