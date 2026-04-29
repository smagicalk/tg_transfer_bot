#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration
/// # Arguments
/// * `first_name` - The first name of the user; 1-64 characters
/// * `last_name` - The last name of the user; 0-64 characters
/// * `disable_notification` - Pass true to disable notification about the current user joining Telegram for other users that added them to contact list
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn register_user(first_name: String, last_name: String, disable_notification: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "registerUser",
        "first_name": first_name,
        "last_name": last_name,
        "disable_notification": disable_notification,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
