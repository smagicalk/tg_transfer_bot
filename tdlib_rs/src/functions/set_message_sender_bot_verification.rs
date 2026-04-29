#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the verification status of a user or a chat by an owned bot
/// # Arguments
/// * `bot_user_id` - Identifier of the owned bot, which will verify the user or the chat
/// * `verified_id` - Identifier of the user or the supergroup or channel chat, which will be verified by the bot
/// * `custom_description` - Custom description of verification reason; 0-getOption("bot_verification_custom_description_length_max").
    /// If empty, then "was verified by organization "organization_name"" will be used as description. Can be specified only if the bot is allowed to provide custom description
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_message_sender_bot_verification(bot_user_id: i64, verified_id: crate::enums::MessageSender, custom_description: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMessageSenderBotVerification",
        "bot_user_id": bot_user_id,
        "verified_id": verified_id,
        "custom_description": custom_description,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
