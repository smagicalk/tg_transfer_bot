#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes order of gift collections. If the collections are owned by a channel chat, then requires can_post_messages administrator right in the channel chat
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_ids` - New order of gift collections
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_gift_collections(owner_id: crate::enums::MessageSender, collection_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderGiftCollections",
        "owner_id": owner_id,
        "collection_ids": collection_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
