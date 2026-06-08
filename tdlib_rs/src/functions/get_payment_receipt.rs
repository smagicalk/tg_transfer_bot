use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a successful payment
/// # Arguments
/// * `chat_id` - Chat identifier of the messagePaymentSuccessful message
/// * `message_id` - Message identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_payment_receipt(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<crate::enums::PaymentReceipt, crate::types::Error> {
    let request = json!({
    "@type": "getPaymentReceipt",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
