use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether aggressive anti-spam checks are enabled in the supergroup. Can be called only if supergroupFullInfo.can_toggle_aggressive_anti_spam == true
/// # Arguments
/// * `supergroup_id` - The identifier of the supergroup, which isn't a broadcast group
/// * `has_aggressive_anti_spam_enabled` - The new value of has_aggressive_anti_spam_enabled
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_has_aggressive_anti_spam_enabled(
    supergroup_id: i64,
    has_aggressive_anti_spam_enabled: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleSupergroupHasAggressiveAntiSpamEnabled",
    "supergroup_id": supergroup_id,
    "has_aggressive_anti_spam_enabled": has_aggressive_anti_spam_enabled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
