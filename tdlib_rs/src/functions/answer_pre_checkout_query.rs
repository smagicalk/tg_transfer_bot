use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the result of a pre-checkout query; for bots only
/// # Arguments
/// * `pre_checkout_query_id` - Identifier of the pre-checkout query
/// * `error_message` - An error message, empty on success
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn answer_pre_checkout_query(
    pre_checkout_query_id: i64,
    error_message: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "answerPreCheckoutQuery",
    "pre_checkout_query_id": pre_checkout_query_id,
    "error_message": error_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
