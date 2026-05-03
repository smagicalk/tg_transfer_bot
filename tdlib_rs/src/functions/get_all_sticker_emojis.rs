use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns unique emoji that correspond to stickers to be found by the getStickers(sticker_type, query, 1000000, chat_id)
/// # Arguments
/// * `sticker_type` - Type of the stickers to search for
/// * `query` - Search query
/// * `chat_id` - Chat identifier for which to find stickers
/// * `return_only_main_emoji` - Pass true if only main emoji for each found sticker must be included in the result
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_all_sticker_emojis(
    sticker_type: crate::enums::StickerType,
    query: String,
    chat_id: i64,
    return_only_main_emoji: bool,
    client_id: i32,
) -> Result<crate::enums::Emojis, crate::types::Error> {
    let request = json!({
    "@type": "getAllStickerEmojis",
    "sticker_type": sticker_type,
    "query": query,
    "chat_id": chat_id,
    "return_only_main_emoji": return_only_main_emoji,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
