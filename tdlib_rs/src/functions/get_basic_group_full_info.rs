#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns full information about a basic group by its identifier
/// # Arguments
/// * `basic_group_id` - Basic group identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_basic_group_full_info(basic_group_id: i64, client_id: i32) -> Result<crate::enums::BasicGroupFullInfo, crate::types::Error> {
    let request = json!({
        "@type": "getBasicGroupFullInfo",
        "basic_group_id": basic_group_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
