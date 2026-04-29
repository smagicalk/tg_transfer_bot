#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Connects an affiliate program to the given affiliate. Returns information about the connected affiliate program
/// # Arguments
/// * `affiliate` - The affiliate to which the affiliate program will be connected
/// * `bot_user_id` - Identifier of the bot, which affiliate program is connected
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn connect_affiliate_program(affiliate: crate::enums::AffiliateType, bot_user_id: i64, client_id: i32) -> Result<crate::enums::ConnectedAffiliateProgram, crate::types::Error> {
    let request = json!({
        "@type": "connectAffiliateProgram",
        "affiliate": affiliate,
        "bot_user_id": bot_user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
