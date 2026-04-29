#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns collections of gifts owned by the given user or chat
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that received the gifts
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_collections(owner_id: crate::enums::MessageSender, client_id: i32) -> Result<crate::enums::GiftCollections, crate::types::Error> {
    let request = json!({
        "@type": "getGiftCollections",
        "owner_id": owner_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
