#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed.
/// If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message
/// # Arguments
/// * `chat_id` - Identifier of the chat to send messages
/// * `message_ids` - Identifiers of the messages to resend. Message identifiers must be in a strictly increasing order
/// * `quote` - New manually chosen quote from the message to be replied; pass null if none. Ignored if more than one message is re-sent, or if messageSendingStateFailed.need_another_reply_quote == false
/// * `paid_message_star_count` - The number of Telegram Stars the user agreed to pay to send the messages. Ignored if messageSendingStateFailed.required_paid_message_star_count == 0
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn resend_messages(chat_id: i64, message_ids: Vec<i64>, quote: Option<crate::types::InputTextQuote>, paid_message_star_count: i64, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "resendMessages",
        "chat_id": chat_id,
        "message_ids": message_ids,
        "quote": quote,
        "paid_message_star_count": paid_message_star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
