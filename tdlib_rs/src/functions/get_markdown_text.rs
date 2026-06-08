use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Replaces text entities with Markdown formatting in a human-friendly format. Entities that can't be represented in Markdown unambiguously are kept as is. Can be called synchronously
/// # Arguments
/// * `text` - The text
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_markdown_text(
    text: crate::types::FormattedText,
    client_id: i32,
) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
    "@type": "getMarkdownText",
    "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
