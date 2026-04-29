#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns default list of custom emoji stickers for placing on a chat photo
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_default_chat_photo_custom_emoji_stickers(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultChatPhotoCustomEmojiStickers",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
