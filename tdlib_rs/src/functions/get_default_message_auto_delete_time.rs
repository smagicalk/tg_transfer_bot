use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns default message auto-delete time setting for new chats
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_default_message_auto_delete_time(
    client_id: i32,
) -> Result<crate::enums::MessageAutoDeleteTime, crate::types::Error> {
    let request = json!({
    "@type": "getDefaultMessageAutoDeleteTime",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
