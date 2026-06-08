use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds or changes business bot that is connected to the current user account
/// # Arguments
/// * `bot` - Connection settings for the bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_connected_bot(
    bot: crate::types::BusinessConnectedBot,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessConnectedBot",
    "bot": bot,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
