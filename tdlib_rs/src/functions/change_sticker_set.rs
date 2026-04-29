#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Installs/uninstalls or activates/archives a sticker set
/// # Arguments
/// * `set_id` - Identifier of the sticker set
/// * `is_installed` - The new value of is_installed
/// * `is_archived` - The new value of is_archived. A sticker set can't be installed and archived simultaneously
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn change_sticker_set(set_id: i64, is_installed: bool, is_archived: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "changeStickerSet",
        "set_id": set_id,
        "is_installed": is_installed,
        "is_archived": is_archived,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
