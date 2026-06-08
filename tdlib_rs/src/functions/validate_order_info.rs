use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Validates the order information provided by a user and returns the available shipping options for a flexible invoice
/// # Arguments
/// * `input_invoice` - The invoice
/// * `order_info` - The order information, provided by the user; pass null if empty
/// * `allow_save` - Pass true to save the order information
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn validate_order_info(
    input_invoice: crate::enums::InputInvoice,
    order_info: Option<crate::types::OrderInfo>,
    allow_save: bool,
    client_id: i32,
) -> Result<crate::enums::ValidatedOrderInfo, crate::types::Error> {
    let request = json!({
    "@type": "validateOrderInfo",
    "input_invoice": input_invoice,
    "order_info": order_info,
    "allow_save": allow_save,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
