#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether sponsored messages are shown in the channel chat; requires owner privileges in the channel. The chat must have at least chatBoostFeatures.min_sponsored_message_disable_boost_level boost level to disable sponsored messages
/// # Arguments
/// * `supergroup_id` - The identifier of the channel
/// * `can_have_sponsored_messages` - The new value of can_have_sponsored_messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_can_have_sponsored_messages(supergroup_id: i64, can_have_sponsored_messages: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupCanHaveSponsoredMessages",
        "supergroup_id": supergroup_id,
        "can_have_sponsored_messages": can_have_sponsored_messages,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
