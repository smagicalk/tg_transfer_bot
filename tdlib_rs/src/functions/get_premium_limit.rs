#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a limit, increased for Premium users. Returns a 404 error if the limit is unknown
/// # Arguments
/// * `limit_type` - Type of the limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_limit(limit_type: crate::enums::PremiumLimitType, client_id: i32) -> Result<crate::enums::PremiumLimit, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumLimit",
        "limit_type": limit_type,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
