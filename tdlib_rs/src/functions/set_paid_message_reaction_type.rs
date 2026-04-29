#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes type of paid message reaction of the current user on a message. The message must have paid reaction added by the current user
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `r#type` - New type of the paid reaction
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_paid_message_reaction_type(chat_id: i64, message_id: i64, r#type: crate::enums::PaidReactionType, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPaidMessageReactionType",
        "chat_id": chat_id,
        "message_id": message_id,
        "type": r#type,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
