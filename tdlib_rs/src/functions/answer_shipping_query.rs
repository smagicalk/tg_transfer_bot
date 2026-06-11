use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the result of a shipping query; for bots only
/// # Arguments
/// * `shipping_query_id` - Identifier of the shipping query
/// * `shipping_options` - Available shipping options
/// * `error_message` - An error message, empty on success
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn answer_shipping_query(
    shipping_query_id: i64,
    shipping_options: Vec<crate::types::ShippingOption>,
    error_message: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "answerShippingQuery",
    "shipping_query_id": shipping_query_id,
    "shipping_options": shipping_options,
    "error_message": error_message,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
