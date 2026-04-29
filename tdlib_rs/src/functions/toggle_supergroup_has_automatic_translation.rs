#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether messages are automatically translated in the channel chat; requires can_change_info administrator right in the channel.
/// The chat must have at least chatBoostFeatures.min_automatic_translation_boost_level boost level to enable automatic translation
/// # Arguments
/// * `supergroup_id` - The identifier of the channel
/// * `has_automatic_translation` - The new value of has_automatic_translation
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_has_automatic_translation(supergroup_id: i64, has_automatic_translation: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupHasAutomaticTranslation",
        "supergroup_id": supergroup_id,
        "has_automatic_translation": has_automatic_translation,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
