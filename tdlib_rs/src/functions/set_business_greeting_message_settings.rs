#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the business greeting message settings of the current user. Requires Telegram Business subscription
/// # Arguments
/// * `greeting_message_settings` - The new settings for the greeting message of the business; pass null to disable the greeting message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_greeting_message_settings(greeting_message_settings: Option<crate::types::BusinessGreetingMessageSettings>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessGreetingMessageSettings",
        "greeting_message_settings": greeting_message_settings,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
