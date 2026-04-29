#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Allows the specified user to send unpaid private messages to the current user by adding a rule to userPrivacySettingAllowUnpaidMessages
/// # Arguments
/// * `user_id` - Identifier of the user
/// * `refund_payments` - Pass true to refund the user previously paid messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn allow_unpaid_messages_from_user(user_id: i64, refund_payments: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "allowUnpaidMessagesFromUser",
        "user_id": user_id,
        "refund_payments": refund_payments,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
