use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Extracts text or caption of the given message and translates it to the given language. If the current user is a Telegram Premium user, then text formatting is preserved
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `to_language_code` - Language code of the language to which the message is translated. See translateText.to_language_code for the list of supported values
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn translate_message_text(
    chat_id: i64,
    message_id: i64,
    to_language_code: String,
    client_id: i32,
) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
    "@type": "translateMessageText",
    "chat_id": chat_id,
    "message_id": message_id,
    "to_language_code": to_language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
