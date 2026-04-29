#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of revenue transactions for a chat. Currently, this method can be used only
/// for channels if supergroupFullInfo.can_get_revenue_statistics == true or bots if userFullInfo.bot_info.can_get_revenue_statistics == true
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `offset` - Offset of the first transaction to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of transactions to be returned; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_revenue_transactions(chat_id: i64, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::ChatRevenueTransactions, crate::types::Error> {
    let request = json!({
        "@type": "getChatRevenueTransactions",
        "chat_id": chat_id,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
