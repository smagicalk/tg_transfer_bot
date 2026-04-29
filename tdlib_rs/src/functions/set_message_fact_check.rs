#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the fact-check of a message. Can be only used if messageProperties.can_set_fact_check == true
/// # Arguments
/// * `chat_id` - The channel chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `text` - New text of the fact-check; 0-getOption("fact_check_length_max") characters; pass null to remove it. Only Bold, Italic, and TextUrl entities with https:t.me/ links are supported
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_message_fact_check(chat_id: i64, message_id: i64, text: Option<crate::types::FormattedText>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setMessageFactCheck",
        "chat_id": chat_id,
        "message_id": message_id,
        "text": text,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
