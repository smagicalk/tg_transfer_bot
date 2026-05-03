use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the period of inactivity after which sessions will automatically be terminated
/// # Arguments
/// * `inactive_session_ttl_days` - New number of days of inactivity before sessions will be automatically terminated; 1-366 days
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_inactive_session_ttl(
    inactive_session_ttl_days: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setInactiveSessionTtl",
    "inactive_session_ttl_days": inactive_session_ttl_days,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
