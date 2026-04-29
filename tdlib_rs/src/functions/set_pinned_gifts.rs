#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the list of pinned gifts on the current user's or the channel's profile page; requires can_post_messages administrator right in the channel chat
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that received the gifts
/// * `received_gift_ids` - New list of pinned gifts. All gifts must be upgraded and saved on the profile page first. There can be up to getOption("pinned_gift_count_max") pinned gifts
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_gifts(owner_id: crate::enums::MessageSender, received_gift_ids: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPinnedGifts",
        "owner_id": owner_id,
        "received_gift_ids": received_gift_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
