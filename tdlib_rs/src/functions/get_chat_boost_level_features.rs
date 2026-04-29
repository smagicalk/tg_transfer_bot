#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of features available on the specific chat boost level. This is an offline method
/// # Arguments
/// * `is_channel` - Pass true to get the list of features for channels; pass false to get the list of features for supergroups
/// * `level` - Chat boost level
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_level_features(is_channel: bool, level: i32, client_id: i32) -> Result<crate::enums::ChatBoostLevelFeatures, crate::types::Error> {
    let request = json!({
        "@type": "getChatBoostLevelFeatures",
        "is_channel": is_channel,
        "level": level,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
