use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs server about an in-store purchase of Telegram Premium before authorization. Works only when the current authorization state is authorizationStateWaitPremiumPurchase
/// # Arguments
/// * `transaction` - Information about the transaction
/// * `is_restore` - Pass true if this is a restore of a Telegram Premium purchase; only for App Store
/// * `currency` - ISO 4217 currency code of the payment currency
/// * `amount` - Paid amount, in the smallest units of the currency
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_authentication_premium_purchase_transaction(
    transaction: crate::enums::StoreTransaction,
    is_restore: bool,
    currency: String,
    amount: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setAuthenticationPremiumPurchaseTransaction",
    "transaction": transaction,
    "is_restore": is_restore,
    "currency": currency,
    "amount": amount,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
