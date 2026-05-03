use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a user who can be contacted to get support
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_support_user(client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
    "@type": "getSupportUser",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
