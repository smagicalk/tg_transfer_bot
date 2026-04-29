#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber
/// # Arguments
/// * `user_id` - Identifier of the user with whom to share the phone number. The user must be a mutual contact
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn share_phone_number(user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sharePhoneNumber",
        "user_id": user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
