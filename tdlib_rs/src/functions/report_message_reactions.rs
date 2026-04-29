#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports reactions set on a message to the Telegram moderators. Reactions on a message can be reported only if messageProperties.can_report_reactions
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_id` - Message identifier
/// * `sender_id` - Identifier of the sender, which added the reaction
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_message_reactions(chat_id: i64, message_id: i64, sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportMessageReactions",
        "chat_id": chat_id,
        "message_id": message_id,
        "sender_id": sender_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
