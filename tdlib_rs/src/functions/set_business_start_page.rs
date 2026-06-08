use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the business start page of the current user. Requires Telegram Business subscription
/// # Arguments
/// * `start_page` - The new start page of the business; pass null to remove custom start page
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_start_page(
    start_page: Option<crate::types::InputBusinessStartPage>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessStartPage",
    "start_page": start_page,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
