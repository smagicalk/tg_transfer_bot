use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the list of close friends of the current user
/// # Arguments
/// * `user_ids` - User identifiers of close friends; the users must be contacts of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_close_friends(
    user_ids: Vec<i64>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setCloseFriends",
    "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
