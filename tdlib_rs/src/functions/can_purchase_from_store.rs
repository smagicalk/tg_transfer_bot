use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks whether an in-store purchase is possible. Must be called before any in-store purchase. For official applications only
/// # Arguments
/// * `purpose` - Transaction purpose
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn can_purchase_from_store(
    purpose: crate::enums::StorePaymentPurpose,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "canPurchaseFromStore",
    "purpose": purpose,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
