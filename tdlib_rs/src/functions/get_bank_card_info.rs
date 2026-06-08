use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a bank card
/// # Arguments
/// * `bank_card_number` - The bank card number
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_bank_card_info(
    bank_card_number: String,
    client_id: i32,
) -> Result<crate::enums::BankCardInfo, crate::types::Error> {
    let request = json!({
    "@type": "getBankCardInfo",
    "bank_card_number": bank_card_number,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
