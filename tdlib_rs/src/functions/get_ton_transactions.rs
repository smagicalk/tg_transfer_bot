use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of Toncoin transactions of the current user
/// # Arguments
/// * `direction` - Direction of the transactions to receive; pass null to get all transactions
/// * `offset` - Offset of the first transaction to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of transactions to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_ton_transactions(
    direction: Option<crate::enums::TransactionDirection>,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::TonTransactions, crate::types::Error> {
    let request = json!({
    "@type": "getTonTransactions",
    "direction": direction,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
