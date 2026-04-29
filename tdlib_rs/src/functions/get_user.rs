#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a user by their identifier. This is an offline method if the current user is not a bot
/// # Arguments
/// * `user_id` - User identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_user(user_id: i64, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getUser",
        "user_id": user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
