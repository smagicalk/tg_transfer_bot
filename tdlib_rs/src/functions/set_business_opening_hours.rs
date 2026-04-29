#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the business opening hours of the current user. Requires Telegram Business subscription
/// # Arguments
/// * `opening_hours` - The new opening hours of the business; pass null to remove the opening hours; up to 28 time intervals can be specified
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_opening_hours(opening_hours: Option<crate::types::BusinessOpeningHours>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessOpeningHours",
        "opening_hours": opening_hours,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
