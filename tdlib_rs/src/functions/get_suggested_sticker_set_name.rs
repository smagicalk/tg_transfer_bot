#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a suggested name for a new sticker set with a given title
/// # Arguments
/// * `title` - Sticker set title; 1-64 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_suggested_sticker_set_name(title: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getSuggestedStickerSetName",
        "title": title,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
