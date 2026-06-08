use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of installed sticker sets
/// # Arguments
/// * `sticker_type` - Type of the sticker sets to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_installed_sticker_sets(
    sticker_type: crate::enums::StickerType,
    client_id: i32,
) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
    "@type": "getInstalledStickerSets",
    "sticker_type": sticker_type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
