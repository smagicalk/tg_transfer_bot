use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes a gift collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_gift_collection(
    owner_id: crate::enums::MessageSender,
    collection_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteGiftCollection",
    "owner_id": owner_id,
    "collection_id": collection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
