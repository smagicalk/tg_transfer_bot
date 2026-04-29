#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that the user viewed detailed information about a Premium feature on the Premium features screen
/// # Arguments
/// * `feature` - The viewed premium feature
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn view_premium_feature(feature: crate::enums::PremiumFeature, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewPremiumFeature",
        "feature": feature,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
