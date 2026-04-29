#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends a filled-out payment form to the bot for final verification
/// # Arguments
/// * `input_invoice` - The invoice
/// * `payment_form_id` - Payment form identifier returned by getPaymentForm
/// * `order_info_id` - Identifier returned by validateOrderInfo, or an empty string
/// * `shipping_option_id` - Identifier of a chosen shipping option, if applicable
/// * `credentials` - The credentials chosen by user for payment; pass null for a payment in Telegram Stars
/// * `tip_amount` - Chosen by the user amount of tip in the smallest units of the currency
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_payment_form(input_invoice: crate::enums::InputInvoice, payment_form_id: i64, order_info_id: String, shipping_option_id: String, credentials: Option<crate::enums::InputCredentials>, tip_amount: i64, client_id: i32) -> Result<crate::enums::PaymentResult, crate::types::Error> {
    let request = json!({
        "@type": "sendPaymentForm",
        "input_invoice": input_invoice,
        "payment_form_id": payment_form_id,
        "order_info_id": order_info_id,
        "shipping_option_id": shipping_option_id,
        "credentials": credentials,
        "tip_amount": tip_amount,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
