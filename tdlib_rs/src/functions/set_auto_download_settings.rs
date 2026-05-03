use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets auto-download settings
/// # Arguments
/// * `settings` - New user auto-download settings
/// * `r#type` - Type of the network for which the new settings are relevant
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_auto_download_settings(
    settings: crate::types::AutoDownloadSettings,
    r#type: crate::enums::NetworkType,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setAutoDownloadSettings",
    "settings": settings,
    "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
