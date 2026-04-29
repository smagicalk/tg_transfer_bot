#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns default list of custom emoji stickers for reply background
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_default_background_custom_emoji_stickers(client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getDefaultBackgroundCustomEmojiStickers",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
