#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns detailed Toncoin revenue statistics of the current user
/// # Arguments
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_ton_revenue_statistics(is_dark: bool, client_id: i32) -> Result<crate::enums::TonRevenueStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getTonRevenueStatistics",
        "is_dark": is_dark,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
