use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets a sticker set thumbnail
/// # Arguments
/// * `user_id` - Sticker set owner; ignored for regular users
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `thumbnail` - Thumbnail to set; pass null to remove the sticker set thumbnail
/// * `format` - Format of the thumbnail; pass null if thumbnail is removed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_set_thumbnail(
    user_id: i64,
    name: String,
    thumbnail: Option<crate::enums::InputFile>,
    format: Option<crate::enums::StickerFormat>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setStickerSetThumbnail",
    "user_id": user_id,
    "name": name,
    "thumbnail": thumbnail,
    "format": format,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
