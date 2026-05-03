use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of Telegram Star transactions for the specified owner
/// # Arguments
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be the identifier of the current user, identifier of an owned bot,
/// or identifier of a supergroup or a channel chat with supergroupFullInfo.can_get_star_revenue_statistics == true
/// * `subscription_id` - If non-empty, only transactions related to the Star Subscription will be returned
/// * `direction` - Direction of the transactions to receive; pass null to get all transactions
/// * `offset` - Offset of the first transaction to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of transactions to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_star_transactions(
    owner_id: crate::enums::MessageSender,
    subscription_id: String,
    direction: Option<crate::enums::TransactionDirection>,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::StarTransactions, crate::types::Error> {
    let request = json!({
    "@type": "getStarTransactions",
    "owner_id": owner_id,
    "subscription_id": subscription_id,
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
