#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a list of sticker sets attached to a file, including regular, mask, and emoji sticker sets. Currently, only animations, photos, and videos can have attached sticker sets
/// # Arguments
/// * `file_id` - File identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_attached_sticker_sets(file_id: i32, client_id: i32) -> Result<crate::enums::StickerSets, crate::types::Error> {
    let request = json!({
        "@type": "getAttachedStickerSets",
        "file_id": file_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
