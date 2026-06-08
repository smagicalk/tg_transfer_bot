use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns all entities (mentions, hashtags, cashtags, bot commands, bank card numbers, URLs, and email addresses) found in the text. Can be called synchronously
/// # Arguments
/// * `text` - The text in which to look for entities
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_text_entities(
    text: String,
    client_id: i32,
) -> Result<crate::enums::TextEntities, crate::types::Error> {
    let request = json!({
    "@type": "getTextEntities",
    "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
