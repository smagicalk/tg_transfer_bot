use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription
/// # Arguments
/// * `device_token` - Device token
/// * `other_user_ids` - List of user identifiers of other users currently using the application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn register_device(
    device_token: crate::enums::DeviceToken,
    other_user_ids: Vec<i64>,
    client_id: i32,
) -> Result<crate::enums::PushReceiverId, crate::types::Error> {
    let request = json!({
    "@type": "registerDevice",
    "device_token": device_token,
    "other_user_ids": other_user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
