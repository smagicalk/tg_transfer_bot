#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the user answer to a poll. A poll in quiz mode can be answered only once
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll
/// * `option_ids` - 0-based identifiers of answer options, chosen by the user. User can choose more than 1 answer option only is the poll allows multiple answers
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_poll_answer(chat_id: i64, message_id: i64, option_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPollAnswer",
        "chat_id": chat_id,
        "message_id": message_id,
        "option_ids": option_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
