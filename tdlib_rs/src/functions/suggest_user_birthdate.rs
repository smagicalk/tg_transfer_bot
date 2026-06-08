use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Suggests a birthdate to another regular user with common messages and allowing non-paid messages
/// # Arguments
/// * `user_id` - User identifier
/// * `birthdate` - Birthdate to suggest
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn suggest_user_birthdate(
    user_id: i64,
    birthdate: crate::types::Birthdate,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "suggestUserBirthdate",
    "user_id": user_id,
    "birthdate": birthdate,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
