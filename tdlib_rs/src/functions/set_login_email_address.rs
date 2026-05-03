use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the login email address of the user. The email address can be changed only if the current user already has login email and passwordState.login_email_address_pattern is non-empty,
/// or the user received suggestedActionSetLoginEmailAddress and isLoginEmailAddressRequired succeeds. The change will not be applied until the new login email address is confirmed with checkLoginEmailAddressCode.
/// To use Apple ID/Google ID instead of an email address, call checkLoginEmailAddressCode directly
/// # Arguments
/// * `new_login_email_address` - New login email address
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_login_email_address(
    new_login_email_address: String,
    client_id: i32,
) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
    "@type": "setLoginEmailAddress",
    "new_login_email_address": new_login_email_address,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
