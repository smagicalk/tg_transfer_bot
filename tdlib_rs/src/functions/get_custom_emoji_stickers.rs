#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of custom emoji stickers by their identifiers. Stickers are returned in arbitrary order. Only found stickers are returned
/// # Arguments
/// * `custom_emoji_ids` - Identifiers of custom emoji stickers. At most 200 custom emoji stickers can be received simultaneously
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_custom_emoji_stickers(custom_emoji_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getCustomEmojiStickers",
        "custom_emoji_ids": custom_emoji_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
