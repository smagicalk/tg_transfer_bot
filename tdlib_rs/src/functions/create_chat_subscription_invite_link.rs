#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates a new subscription invite link for a channel chat. Requires can_invite_users right in the chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `name` - Invite link name; 0-32 characters
/// * `subscription_pricing` - Information about subscription plan that will be applied to the users joining the chat via the link.
    /// Subscription period must be 2592000 in production environment, and 60 or 300 if Telegram test environment is used
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_chat_subscription_invite_link(chat_id: i64, name: String, subscription_pricing: crate::types::StarSubscriptionPricing, client_id: i32) -> Result<crate::enums::ChatInviteLink, crate::types::Error> {
    let request = json!({
        "@type": "createChatSubscriptionInviteLink",
        "chat_id": chat_id,
        "name": name,
        "subscription_pricing": subscription_pricing,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
