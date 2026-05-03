use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the list of emojis corresponding to a sticker. The sticker must belong to a regular or custom emoji sticker set that is owned by the current user
/// # Arguments
/// * `sticker` - Sticker
/// * `emojis` - New string with 1-20 emoji corresponding to the sticker
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_emojis(
    sticker: crate::enums::InputFile,
    emojis: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setStickerEmojis",
    "sticker": sticker,
    "emojis": emojis,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
