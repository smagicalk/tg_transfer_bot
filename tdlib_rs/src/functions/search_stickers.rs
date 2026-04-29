#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches for stickers from public sticker sets that correspond to any of the given emoji
/// # Arguments
/// * `sticker_type` - Type of the stickers to return
/// * `emojis` - Space-separated list of emojis to search for
/// * `query` - Query to search for; may be empty to search for emoji only
/// * `input_language_codes` - List of possible IETF language tags of the user's input language; may be empty if unknown
/// * `offset` - The offset from which to return the stickers; must be non-negative
/// * `limit` - The maximum number of stickers to be returned; 0-100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_stickers(sticker_type: crate::enums::StickerType, emojis: String, query: String, input_language_codes: Vec<String>, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "searchStickers",
        "sticker_type": sticker_type,
        "emojis": emojis,
        "query": query,
        "input_language_codes": input_language_codes,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
