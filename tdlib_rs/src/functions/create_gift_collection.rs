use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a collection from gifts on the current user's or a channel's profile page; requires can_post_messages administrator right in the channel chat.
/// An owner can have up to getOption("gift_collection_count_max") gift collections. The new collection will be added to the end of the gift collection list of the owner. Returns the created collection
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that received the gifts
/// * `name` - Name of the collection; 1-12 characters
/// * `received_gift_ids` - Identifier of the gifts to add to the collection; 0-getOption("gift_collection_size_max") identifiers
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_gift_collection(
    owner_id: crate::enums::MessageSender,
    name: String,
    received_gift_ids: Vec<String>,
    client_id: i32,
) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
    "@type": "createGiftCollection",
    "owner_id": owner_id,
    "name": name,
    "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
