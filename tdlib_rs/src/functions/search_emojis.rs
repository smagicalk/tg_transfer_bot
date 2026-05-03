use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for emojis by keywords. Supported only if the file database is enabled. Order of results is unspecified
/// # Arguments
/// * `text` - Text to search for
/// * `input_language_codes` - List of possible IETF language tags of the user's input language; may be empty if unknown
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_emojis(
    text: String,
    input_language_codes: Vec<String>,
    client_id: i32,
) -> Result<crate::enums::EmojiKeywords, crate::types::Error> {
    let request = json!({
    "@type": "searchEmojis",
    "text": text,
    "input_language_codes": input_language_codes,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
