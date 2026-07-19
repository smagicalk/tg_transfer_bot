use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a link for the given invoice; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request
/// * `invoice` - Information about the invoice of the type inputMessageInvoice
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_invoice_link(
    business_connection_id: String,
    invoice: crate::enums::InputMessageContent,
    client_id: i32,
) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
    "@type": "createInvoiceLink",
    "business_connection_id": business_connection_id,
    "invoice": invoice,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
