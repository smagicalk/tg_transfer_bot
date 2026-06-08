use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns privacy settings for message read date
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_read_date_privacy_settings(
    client_id: i32,
) -> Result<crate::enums::ReadDatePrivacySettings, crate::types::Error> {
    let request = json!({
    "@type": "getReadDatePrivacySettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
