use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Quickly returns approximate storage usage statistics. Can be called before authorization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_storage_statistics_fast(
    client_id: i32,
) -> Result<crate::enums::StorageStatisticsFast, crate::types::Error> {
    let request = json!({
    "@type": "getStorageStatisticsFast",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
