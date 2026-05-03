use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a sticker set by its identifier
/// # Arguments
/// * `set_id` - Identifier of the sticker set
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_sticker_set(
    set_id: i64,
    client_id: i32,
) -> Result<crate::enums::StickerSet, crate::types::Error> {
    let request = json!({
    "@type": "getStickerSet",
    "set_id": set_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
