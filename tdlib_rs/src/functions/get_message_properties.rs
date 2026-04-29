#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns properties of a message. This is an offline method
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_id` - Identifier of the message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_properties(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::MessageProperties, crate::types::Error> {
    let request = json!({
        "@type": "getMessageProperties",
        "chat_id": chat_id,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
