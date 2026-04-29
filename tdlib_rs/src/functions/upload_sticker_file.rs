#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Uploads a file with a sticker; returns the uploaded file
/// # Arguments
/// * `user_id` - Sticker file owner; ignored for regular users
/// * `sticker_format` - Sticker format
/// * `sticker` - File file to upload; must fit in a 512x512 square. For WEBP stickers the file must be in WEBP or PNG format, which will be converted to WEBP server-side.
    /// See https:core.telegram.org/animated_stickers#technical-requirements for technical requirements
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn upload_sticker_file(user_id: i64, sticker_format: crate::enums::StickerFormat, sticker: crate::enums::InputFile, client_id: i32) -> Result<crate::enums::File, crate::types::Error> {
    let request = json!({
        "@type": "uploadStickerFile",
        "user_id": user_id,
        "sticker_format": sticker_format,
        "sticker": sticker,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
