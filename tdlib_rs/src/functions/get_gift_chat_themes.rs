#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns available to the current user gift chat themes
/// # Arguments
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of chat themes to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_chat_themes(offset: String, limit: i32, client_id: i32) -> Result<crate::enums::GiftChatThemes, crate::types::Error> {
    let request = json!({
        "@type": "getGiftChatThemes",
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
