use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether the current user has sponsored messages enabled. The setting has no effect for users without Telegram Premium for which sponsored messages are always enabled
/// # Arguments
/// * `has_sponsored_messages_enabled` - Pass true to enable sponsored messages for the current user; false to disable them
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_has_sponsored_messages_enabled(
    has_sponsored_messages_enabled: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleHasSponsoredMessagesEnabled",
    "has_sponsored_messages_enabled": has_sponsored_messages_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
