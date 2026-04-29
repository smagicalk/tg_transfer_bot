#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for a given quote in a text. Returns found quote start position in UTF-16 code units. Returns a 404 error if the quote is not found. Can be called synchronously
/// # Arguments
/// * `text` - Text in which to search for the quote
/// * `quote` - Quote to search for
/// * `quote_position` - Approximate quote position in UTF-16 code units
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_quote(text: crate::types::FormattedText, quote: crate::types::FormattedText, quote_position: i32, client_id: i32) -> Result<crate::enums::FoundPosition, crate::types::Error> {
    let request = json!({
        "@type": "searchQuote",
        "text": text,
        "quote": quote,
        "quote_position": quote_position,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
