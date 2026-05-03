use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Disconnects an affiliate program from the given affiliate and immediately deactivates its referral link. Returns updated information about the disconnected affiliate program
/// # Arguments
/// * `affiliate` - The affiliate to which the affiliate program is connected
/// * `url` - The referral link of the affiliate program
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn disconnect_affiliate_program(
    affiliate: crate::enums::AffiliateType,
    url: String,
    client_id: i32,
) -> Result<crate::enums::ConnectedAffiliateProgram, crate::types::Error> {
    let request = json!({
    "@type": "disconnectAffiliateProgram",
    "affiliate": affiliate,
    "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
