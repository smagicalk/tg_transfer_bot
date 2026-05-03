use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an invoice payment form. This method must be called when the user presses inline button of the type inlineKeyboardButtonTypeBuy, or wants to buy access to media in a messagePaidMedia message
/// # Arguments
/// * `input_invoice` - The invoice
/// * `theme` - Preferred payment form theme; pass null to use the default theme
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_payment_form(
    input_invoice: crate::enums::InputInvoice,
    theme: Option<crate::types::ThemeParameters>,
    client_id: i32,
) -> Result<crate::enums::PaymentForm, crate::types::Error> {
    let request = json!({
    "@type": "getPaymentForm",
    "input_invoice": input_invoice,
    "theme": theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
