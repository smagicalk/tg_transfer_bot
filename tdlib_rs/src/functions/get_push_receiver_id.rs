#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. Can be called synchronously
/// # Arguments
/// * `payload` - JSON-encoded push notification payload
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_push_receiver_id(payload: String, client_id: i32) -> Result<crate::enums::PushReceiverId, crate::types::Error> {
    let request = json!({
        "@type": "getPushReceiverId",
        "payload": payload,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
