use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a sticker from the list of favorite stickers
/// # Arguments
/// * `sticker` - Sticker file to delete from the list
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_favorite_sticker(
    sticker: crate::enums::InputFile,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeFavoriteSticker",
    "sticker": sticker,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
