#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks the login email address authentication
/// # Arguments
/// * `code` - Email address authentication to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_login_email_address_code(code: crate::enums::EmailAddressAuthentication, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkLoginEmailAddressCode",
        "code": code,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
