use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns full information about a user by their identifier
/// # Arguments
/// * `user_id` - User identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_user_full_info(
    user_id: i64,
    client_id: i32,
) -> Result<crate::enums::UserFullInfo, crate::types::Error> {
    let request = json!({
    "@type": "getUserFullInfo",
    "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
