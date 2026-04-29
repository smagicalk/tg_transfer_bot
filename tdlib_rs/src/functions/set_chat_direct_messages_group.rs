#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes direct messages group settings for a channel chat; requires owner privileges in the chat
/// # Arguments
/// * `chat_id` - Identifier of the channel chat
/// * `is_enabled` - Pass true if the direct messages group is enabled for the channel chat; pass false otherwise
/// * `paid_message_star_count` - The new number of Telegram Stars that must be paid for each message that is sent to the direct messages chat unless the sender is an administrator of the channel chat; 0-getOption("paid_message_star_count_max").
    /// The channel will receive getOption("paid_message_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for message sending. Requires supergroupFullInfo.can_enable_paid_messages for positive amounts
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_direct_messages_group(chat_id: i64, is_enabled: bool, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDirectMessagesGroup",
        "chat_id": chat_id,
        "is_enabled": is_enabled,
        "paid_message_star_count": paid_message_star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
