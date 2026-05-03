use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns affiliate programs that were connected to the given affiliate
/// # Arguments
/// * `affiliate` - The affiliate to which the affiliate program were connected
/// * `offset` - Offset of the first affiliate program to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of affiliate programs to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_connected_affiliate_programs(
    affiliate: crate::enums::AffiliateType,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::ConnectedAffiliatePrograms, crate::types::Error> {
    let request = json!({
    "@type": "getConnectedAffiliatePrograms",
    "affiliate": affiliate,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
