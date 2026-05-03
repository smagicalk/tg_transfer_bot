use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the list of keywords of a sticker. The sticker must belong to a regular or custom emoji sticker set that is owned by the current user
/// # Arguments
/// * `sticker` - Sticker
/// * `keywords` - List of up to 20 keywords with total length up to 64 characters, which can be used to find the sticker
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_keywords(
    sticker: crate::enums::InputFile,
    keywords: Vec<String>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setStickerKeywords",
    "sticker": sticker,
    "keywords": keywords,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
