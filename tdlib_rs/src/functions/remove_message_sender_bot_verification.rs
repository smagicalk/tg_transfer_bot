#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes the verification status of a user or a chat by an owned bot
/// # Arguments
/// * `bot_user_id` - Identifier of the owned bot, which verified the user or the chat
/// * `verified_id` - Identifier of the user or the supergroup or channel chat, which verification is removed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_message_sender_bot_verification(bot_user_id: i64, verified_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeMessageSenderBotVerification",
        "bot_user_id": bot_user_id,
        "verified_id": verified_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
