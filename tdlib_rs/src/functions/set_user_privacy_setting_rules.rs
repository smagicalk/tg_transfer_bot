use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes user privacy settings
/// # Arguments
/// * `setting` - The privacy setting
/// * `rules` - The new privacy rules
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_user_privacy_setting_rules(
    setting: crate::enums::UserPrivacySetting,
    rules: crate::types::UserPrivacySettingRules,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setUserPrivacySettingRules",
    "setting": setting,
    "rules": rules,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
