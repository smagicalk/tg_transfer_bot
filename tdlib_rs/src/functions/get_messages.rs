#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about messages. If a message is not found, returns null on the corresponding position of the result
/// # Arguments
/// * `chat_id` - Identifier of the chat the messages belong to
/// * `message_ids` - Identifiers of the messages to get
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_messages(chat_id: i64, message_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getMessages",
        "chat_id": chat_id,
        "message_ids": message_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
