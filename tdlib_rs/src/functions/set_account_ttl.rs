use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the period of inactivity after which the account of the current user will automatically be deleted
/// # Arguments
/// * `ttl` - New account TTL
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_account_ttl(
    ttl: crate::types::AccountTtl,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setAccountTtl",
    "ttl": ttl,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
