#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about features, available to Premium users
/// # Arguments
/// * `source` - Source of the request; pass null if the method is called from some non-standard source
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_features(source: Option<crate::enums::PremiumSource>, client_id: i32) -> Result<crate::enums::PremiumFeatures, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumFeatures",
        "source": source,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
