use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Summarizes content of the message with non-empty summary_language_code
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `translate_to_language_code` - Pass a language code to which the summary will be translated; may be empty if translation isn't needed. See translateText.to_language_code for the list of supported values
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn summarize_message(
    chat_id: i64,
    message_id: i64,
    translate_to_language_code: String,
    client_id: i32,
) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
    "@type": "summarizeMessage",
    "chat_id": chat_id,
    "message_id": message_id,
    "translate_to_language_code": translate_to_language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
