use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes name of a gift collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat. Returns the changed collection
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `name` - New name of the collection; 1-12 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_gift_collection_name(
    owner_id: crate::enums::MessageSender,
    collection_id: i32,
    name: String,
    client_id: i32,
) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
    "@type": "setGiftCollectionName",
    "owner_id": owner_id,
    "collection_id": collection_id,
    "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
