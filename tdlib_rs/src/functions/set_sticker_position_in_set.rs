use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the position of a sticker in the set to which it belongs. The sticker set must be owned by the current user
/// # Arguments
/// * `sticker` - Sticker
/// * `position` - New position of the sticker in the set, 0-based
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_position_in_set(
    sticker: crate::enums::InputFile,
    position: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setStickerPositionInSet",
    "sticker": sticker,
    "position": position,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
