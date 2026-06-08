use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the current privacy settings
/// # Arguments
/// * `setting` - The privacy setting
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_user_privacy_setting_rules(
    setting: crate::enums::UserPrivacySetting,
    client_id: i32,
) -> Result<crate::enums::UserPrivacySettingRules, crate::types::Error> {
    let request = json!({
    "@type": "getUserPrivacySettingRules",
    "setting": setting,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
