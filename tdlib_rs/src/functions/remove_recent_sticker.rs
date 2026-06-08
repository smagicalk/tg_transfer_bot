use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a sticker from the list of recently used stickers
/// # Arguments
/// * `is_attached` - Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers
/// * `sticker` - Sticker file to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_recent_sticker(
    is_attached: bool,
    sticker: crate::enums::InputFile,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeRecentSticker",
    "is_attached": is_attached,
    "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
