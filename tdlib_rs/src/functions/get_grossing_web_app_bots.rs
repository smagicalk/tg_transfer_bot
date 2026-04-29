#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the most grossing Web App bots
/// # Arguments
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of bots to be returned; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_grossing_web_app_bots(offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundUsers, crate::types::Error> {
    let request = json!({
        "@type": "getGrossingWebAppBots",
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
