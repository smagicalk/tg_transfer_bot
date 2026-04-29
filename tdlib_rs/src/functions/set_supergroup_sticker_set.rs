#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the sticker set of a supergroup; requires can_change_info administrator right
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup
/// * `sticker_set_id` - New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_sticker_set(supergroup_id: i64, sticker_set_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupStickerSet",
        "supergroup_id": supergroup_id,
        "sticker_set_id": sticker_set_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
