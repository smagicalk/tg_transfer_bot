use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account.
/// Can be called before authorization when the current authorization state is authorizationStateWaitPassword
/// # Arguments
/// * `reason` - The reason why the account was deleted; optional
/// * `password` - The 2-step verification password of the current user. If the current user isn't authorized, then an empty string can be passed and account deletion can be canceled within one week
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_account(
    reason: String,
    password: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteAccount",
    "reason": reason,
    "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
