#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a Telegram Premium gift code
/// # Arguments
/// * `code` - The code to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_premium_gift_code(code: String, client_id: i32) -> Result<crate::enums::PremiumGiftCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "checkPremiumGiftCode",
        "code": code,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
