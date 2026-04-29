#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets the email address of the user and sends an authentication code to the email address. Works only when the current authorization state is authorizationStateWaitEmailAddress
/// # Arguments
/// * `email_address` - The email address of the user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_authentication_email_address(email_address: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAuthenticationEmailAddress",
        "email_address": email_address,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
