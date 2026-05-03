use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets autosave settings for the given scope. The method is guaranteed to work only after at least one call to getAutosaveSettings
/// # Arguments
/// * `scope` - Autosave settings scope
/// * `settings` - New autosave settings for the scope; pass null to set autosave settings to default
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_autosave_settings(
    scope: crate::enums::AutosaveSettingsScope,
    settings: Option<crate::types::ScopeAutosaveSettings>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setAutosaveSettings",
    "scope": scope,
    "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
