use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a new call
/// # Arguments
/// * `user_id` - Identifier of the user to be called
/// * `protocol` - The call protocols supported by the application
/// * `is_video` - Pass true to create a video call
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_call(
    user_id: i64,
    protocol: crate::types::CallProtocol,
    is_video: bool,
    client_id: i32,
) -> Result<crate::enums::CallId, crate::types::Error> {
    let request = json!({
    "@type": "createCall",
    "user_id": user_id,
    "protocol": protocol,
    "is_video": is_video,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
