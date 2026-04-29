#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in
/// # Arguments
/// * `token` - The bot token
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_bot_token(token: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationBotToken",
        "token": token,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
