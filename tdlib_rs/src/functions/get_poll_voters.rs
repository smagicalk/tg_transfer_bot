use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns message senders voted for the specified option in a non-anonymous polls. For optimal performance, the number of returned users is chosen by TDLib
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll
/// * `option_id` - 0-based identifier of the answer option
/// * `offset` - Number of voters to skip in the result; must be non-negative
/// * `limit` - The maximum number of voters to be returned; must be positive and can't be greater than 50. For optimal performance, the number of returned voters is chosen by TDLib and can be smaller than the specified limit, even if the end of the voter list has not been reached
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_poll_voters(
    chat_id: i64,
    message_id: i64,
    option_id: i32,
    offset: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::PollVoters, crate::types::Error> {
    let request = json!({
    "@type": "getPollVoters",
    "chat_id": chat_id,
    "message_id": message_id,
    "option_id": option_id,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
