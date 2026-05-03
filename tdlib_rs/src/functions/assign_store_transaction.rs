use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs server about an in-store purchase. For official applications only
/// # Arguments
/// * `transaction` - Information about the transaction
/// * `purpose` - Transaction purpose
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn assign_store_transaction(
    transaction: crate::enums::StoreTransaction,
    purpose: crate::enums::StorePaymentPurpose,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "assignStoreTransaction",
    "transaction": transaction,
    "purpose": purpose,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
