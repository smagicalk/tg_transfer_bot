use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation
/// # Arguments
/// * `language_code` - Language code for which the emoji replacements will be suggested
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_emoji_suggestions_url(
    language_code: String,
    client_id: i32,
) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
    "@type": "getEmojiSuggestionsUrl",
    "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
