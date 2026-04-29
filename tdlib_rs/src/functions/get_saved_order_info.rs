#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns saved order information. Returns a 404 error if there is no saved order information
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_order_info(client_id: i32) -> Result<crate::enums::OrderInfo, crate::types::Error> {
    let request = json!({
        "@type": "getSavedOrderInfo",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
