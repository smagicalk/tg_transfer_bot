use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Clears the list of all autosave settings exceptions. The method is guaranteed to work only after at least one call to getAutosaveSettings
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn clear_autosave_settings_exceptions(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "clearAutosaveSettingsExceptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
