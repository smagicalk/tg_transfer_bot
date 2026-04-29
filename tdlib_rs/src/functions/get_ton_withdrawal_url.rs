#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a URL for Toncoin withdrawal from the current user's account. The user must have at least 10 toncoins to withdraw
/// and can withdraw up to 100000 Toncoins in one transaction
/// # Arguments
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_ton_withdrawal_url(password: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getTonWithdrawalUrl",
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
