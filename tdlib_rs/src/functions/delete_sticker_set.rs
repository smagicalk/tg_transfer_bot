#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Completely deletes a sticker set
/// # Arguments
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_sticker_set(name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteStickerSet",
        "name": name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
