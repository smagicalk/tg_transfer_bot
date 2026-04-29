#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Parses Markdown entities in a human-friendly format, ignoring markup errors. Can be called synchronously
/// # Arguments
/// * `text` - The text to parse. For example, "__italic__ ~~strikethrough~~ ||spoiler|| **bold** `code` ```pre``` __[italic__ text_url](telegram.org) __italic**bold italic__bold**"
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn parse_markdown(text: crate::types::FormattedText, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "parseMarkdown",
        "text": text,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
