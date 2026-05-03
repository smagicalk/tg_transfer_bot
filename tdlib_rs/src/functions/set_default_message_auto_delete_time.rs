use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the default message auto-delete time for new chats
/// # Arguments
/// * `message_auto_delete_time` - New default message auto-delete time; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_default_message_auto_delete_time(
    message_auto_delete_time: crate::types::MessageAutoDeleteTime,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setDefaultMessageAutoDeleteTime",
    "message_auto_delete_time": message_auto_delete_time,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
