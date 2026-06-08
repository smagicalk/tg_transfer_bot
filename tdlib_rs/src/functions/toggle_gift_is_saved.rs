use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether a gift is shown on the current user's or the channel's profile page; requires can_post_messages administrator right in the channel chat
/// # Arguments
/// * `received_gift_id` - Identifier of the gift
/// * `is_saved` - Pass true to display the gift on the user's or the channel's profile page; pass false to remove it from the profile page
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_gift_is_saved(
    received_gift_id: String,
    is_saved: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleGiftIsSaved",
    "received_gift_id": received_gift_id,
    "is_saved": is_saved,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
