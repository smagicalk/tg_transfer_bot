use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about features, available to Business users
/// # Arguments
/// * `source` - Source of the request; pass null if the method is called from settings or some non-standard source
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_business_features(
    source: Option<crate::enums::BusinessFeature>,
    client_id: i32,
) -> Result<crate::enums::BusinessFeatures, crate::types::Error> {
    let request = json!({
    "@type": "getBusinessFeatures",
    "source": source,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
