#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Searches affiliate programs that can be connected to the given affiliate
/// # Arguments
/// * `affiliate` - The affiliate for which affiliate programs are searched for
/// * `sort_order` - Sort order for the results
/// * `offset` - Offset of the first affiliate program to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of affiliate programs to return
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_affiliate_programs(affiliate: crate::enums::AffiliateType, sort_order: crate::enums::AffiliateProgramSortOrder, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::FoundAffiliatePrograms, crate::types::Error> {
    let request = json!({
        "@type": "searchAffiliatePrograms",
        "affiliate": affiliate,
        "sort_order": sort_order,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
