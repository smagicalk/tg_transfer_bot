#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location.
/// Returns the edited message after the edit is completed on the server side
/// # Arguments
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_be_edited to check whether the message can be edited
/// * `location` - New location content of the message; pass null to stop sharing the live location
/// * `live_period` - New time relative to the message send date, for which the location can be updated, in seconds. If 0x7FFFFFFF specified, then the location can be updated forever.
    /// Otherwise, must not exceed the current live_period by more than a day, and the live location expiration date must remain in the next 90 days. Pass 0 to keep the current live_period
/// * `heading` - The new direction in which the location moves, in degrees; 1-360. Pass 0 if unknown
/// * `proximity_alert_radius` - The new maximum distance for proximity alerts, in meters (0-100000). Pass 0 if the notification is disabled
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_live_location(chat_id: i64, message_id: i64, location: Option<crate::types::Location>, live_period: i32, heading: i32, proximity_alert_radius: i32, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "editMessageLiveLocation",
        "chat_id": chat_id,
        "message_id": message_id,
        "location": location,
        "live_period": live_period,
        "heading": heading,
        "proximity_alert_radius": proximity_alert_radius,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
