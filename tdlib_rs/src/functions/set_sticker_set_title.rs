#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets a sticker set title
/// # Arguments
/// * `name` - Sticker set name. The sticker set must be owned by the current user
/// * `title` - New sticker set title
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_set_title(name: String, title: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerSetTitle",
        "name": name,
        "title": title,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
