use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns gifts received by the given user or chat
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `owner_id` - Identifier of the gift receiver
/// * `collection_id` - Pass collection identifier to get gifts only from the specified collection; pass 0 to get gifts regardless of collections
/// * `exclude_unsaved` - Pass true to exclude gifts that aren't saved to the chat's profile page. Always true for gifts received by other users and channel chats without can_post_messages administrator right
/// * `exclude_saved` - Pass true to exclude gifts that are saved to the chat's profile page. Always false for gifts received by other users and channel chats without can_post_messages administrator right
/// * `exclude_unlimited` - Pass true to exclude gifts that can be purchased unlimited number of times
/// * `exclude_upgradable` - Pass true to exclude gifts that can be purchased limited number of times and can be upgraded
/// * `exclude_non_upgradable` - Pass true to exclude gifts that can be purchased limited number of times and can't be upgraded
/// * `exclude_upgraded` - Pass true to exclude upgraded gifts
/// * `exclude_without_colors` - Pass true to exclude gifts that can't be used in setUpgradedGiftColors
/// * `exclude_hosted` - Pass true to exclude gifts that are just hosted and are not owned by the owner
/// * `sort_by_price` - Pass true to sort results by gift price instead of send date
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of gifts to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_received_gifts(
    business_connection_id: String,
    owner_id: crate::enums::MessageSender,
    collection_id: i32,
    exclude_unsaved: bool,
    exclude_saved: bool,
    exclude_unlimited: bool,
    exclude_upgradable: bool,
    exclude_non_upgradable: bool,
    exclude_upgraded: bool,
    exclude_without_colors: bool,
    exclude_hosted: bool,
    sort_by_price: bool,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::ReceivedGifts, crate::types::Error> {
    let request = json!({
    "@type": "getReceivedGifts",
    "business_connection_id": business_connection_id,
    "owner_id": owner_id,
    "collection_id": collection_id,
    "exclude_unsaved": exclude_unsaved,
    "exclude_saved": exclude_saved,
    "exclude_unlimited": exclude_unlimited,
    "exclude_upgradable": exclude_upgradable,
    "exclude_non_upgradable": exclude_non_upgradable,
    "exclude_upgraded": exclude_upgraded,
    "exclude_without_colors": exclude_without_colors,
    "exclude_hosted": exclude_hosted,
    "sort_by_price": sort_by_price,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
