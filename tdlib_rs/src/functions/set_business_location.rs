#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the business location of the current user. Requires Telegram Business subscription
/// # Arguments
/// * `location` - The new location of the business; pass null to remove the location
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_location(location: Option<crate::types::BusinessLocation>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBusinessLocation",
        "location": location,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
