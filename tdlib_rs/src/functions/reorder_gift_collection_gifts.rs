use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes order of gifts in a collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat. Returns the changed collection
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `received_gift_ids` - Identifier of the gifts to move to the beginning of the collection. All other gifts are placed in the current order after the specified gifts
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_gift_collection_gifts(
    owner_id: crate::enums::MessageSender,
    collection_id: i32,
    received_gift_ids: Vec<String>,
    client_id: i32,
) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
    "@type": "reorderGiftCollectionGifts",
    "owner_id": owner_id,
    "collection_id": collection_id,
    "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
