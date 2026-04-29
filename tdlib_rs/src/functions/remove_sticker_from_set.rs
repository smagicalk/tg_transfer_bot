#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes a sticker from the set to which it belongs. The sticker set must be owned by the current user
/// # Arguments
/// * `sticker` - Sticker to remove from the set
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_sticker_from_set(sticker: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeStickerFromSet",
        "sticker": sticker,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
