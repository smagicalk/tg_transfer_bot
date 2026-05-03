use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of features available for different chat boost levels. This is an offline method
/// # Arguments
/// * `is_channel` - Pass true to get the list of features for channels; pass false to get the list of features for supergroups
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_features(
    is_channel: bool,
    client_id: i32,
) -> Result<crate::enums::ChatBoostFeatures, crate::types::Error> {
    let request = json!({
    "@type": "getChatBoostFeatures",
    "is_channel": is_channel,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
