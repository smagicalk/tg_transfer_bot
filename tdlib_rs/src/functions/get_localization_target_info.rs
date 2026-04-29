#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about the current localization target. This is an offline method if only_local is true. Can be called before authorization
/// # Arguments
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_localization_target_info(only_local: bool, client_id: i32) -> Result<crate::enums::LocalizationTargetInfo, crate::types::Error> {
    let request = json!({
        "@type": "getLocalizationTargetInfo",
        "only_local": only_local,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
