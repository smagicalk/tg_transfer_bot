use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Parses Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, BlockQuote, ExpandableBlockQuote, Code, Pre, PreCode, TextUrl
/// and MentionName entities from a marked-up text. Can be called synchronously
/// # Arguments
/// * `text` - The text to parse
/// * `parse_mode` - Text parse mode
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn parse_text_entities(
    text: String,
    parse_mode: crate::enums::TextParseMode,
    client_id: i32,
) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
    "@type": "parseTextEntities",
    "text": text,
    "parse_mode": parse_mode,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
