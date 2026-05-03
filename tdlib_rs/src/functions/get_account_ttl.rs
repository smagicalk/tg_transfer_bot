use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the period of inactivity after which the account of the current user will automatically be deleted
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_account_ttl(
    client_id: i32,
) -> Result<crate::enums::AccountTtl, crate::types::Error> {
    let request = json!({
    "@type": "getAccountTtl",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
