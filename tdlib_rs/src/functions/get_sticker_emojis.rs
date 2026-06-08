use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
/// # Arguments
/// * `sticker` - Sticker file identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_emojis(
    sticker: crate::enums::InputFile,
    client_id: i32,
) -> Result<crate::enums::Emojis, crate::types::Error> {
    let request = json!({
    "@type": "getStickerEmojis",
    "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
