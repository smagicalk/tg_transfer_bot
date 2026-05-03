use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches a user by a token from the user's link
/// # Arguments
/// * `token` - Token to search for
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_user_by_token(
    token: String,
    client_id: i32,
) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
    "@type": "searchUserByToken",
    "token": token,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
