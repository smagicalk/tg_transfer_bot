#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds gifts to the beginning of a previously created collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat. Returns the changed collection
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `received_gift_ids` - Identifier of the gifts to add to the collection; 1-getOption("gift_collection_size_max") identifiers.
    /// If after addition the collection has more than getOption("gift_collection_size_max") gifts, then the last one are removed from the collection
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_gift_collection_gifts(owner_id: crate::enums::MessageSender, collection_id: i32, received_gift_ids: Vec<String>, client_id: i32) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
        "@type": "addGiftCollectionGifts",
        "owner_id": owner_id,
        "collection_id": collection_id,
        "received_gift_ids": received_gift_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
