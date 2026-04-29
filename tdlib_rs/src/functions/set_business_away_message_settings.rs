#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the business away message settings of the current user. Requires Telegram Business subscription
/// # Arguments
/// * `away_message_settings` - The new settings for the away message of the business; pass null to disable the away message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_away_message_settings(away_message_settings: Option<crate::types::BusinessAwayMessageSettings>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessAwayMessageSettings",
        "away_message_settings": away_message_settings,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
