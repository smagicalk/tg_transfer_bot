#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends a message to other participants of a group call. Requires groupCall.can_send_messages right
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `text` - Text of the message to send; 1-getOption("group_call_message_text_length_max") characters for non-live-stories; see updateGroupCallMessageLevels for live story restrictions,
    /// which depends on paid_message_star_count. Can't contain line feeds for live stories
/// * `paid_message_star_count` - The number of Telegram Stars the user agreed to pay to send the message; for live stories only; 0-getOption("paid_group_call_message_star_count_max").
    /// Must be 0 for messages sent to live stories posted by the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_group_call_message(group_call_id: i32, text: crate::types::FormattedText, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendGroupCallMessage",
        "group_call_id": group_call_id,
        "text": text,
        "paid_message_star_count": paid_message_star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
