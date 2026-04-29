#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns stickers from the installed sticker sets that correspond to any of the given emoji or can be found by sticker-specific keywords. If the query is non-empty, then favorite, recently used or trending stickers may also be returned
/// # Arguments
/// * `sticker_type` - Type of the stickers to return
/// * `query` - Search query; a space-separated list of emojis or a keyword prefix. If empty, returns all known installed stickers
/// * `limit` - The maximum number of stickers to be returned
/// * `chat_id` - Chat identifier for which to return stickers. Available custom emoji stickers may be different for different chats
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_stickers(sticker_type: crate::enums::StickerType, query: String, limit: i32, chat_id: i64, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getStickers",
        "sticker_type": sticker_type,
        "query": query,
        "limit": limit,
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
