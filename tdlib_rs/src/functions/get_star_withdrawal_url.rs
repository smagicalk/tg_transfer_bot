#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a URL for Telegram Star withdrawal
/// # Arguments
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be identifier of the current user, an owned bot, or an owned supergroup or channel chat
/// * `star_count` - The number of Telegram Stars to withdraw; must be between getOption("star_withdrawal_count_min") and getOption("star_withdrawal_count_max")
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_star_withdrawal_url(owner_id: crate::enums::MessageSender, star_count: i64, password: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getStarWithdrawalUrl",
        "owner_id": owner_id,
        "star_count": star_count,
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
