#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the order of installed sticker sets
/// # Arguments
/// * `sticker_type` - Type of the sticker sets to reorder
/// * `sticker_set_ids` - Identifiers of installed sticker sets in the new correct order
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_installed_sticker_sets(sticker_type: crate::enums::StickerType, sticker_set_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderInstalledStickerSets",
        "sticker_type": sticker_type,
        "sticker_set_ids": sticker_set_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
