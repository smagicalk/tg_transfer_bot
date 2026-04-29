#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a URL for a Telegram Ad platform account that can be used to set up advertisements for the chat paid in the owned Telegram Stars
/// # Arguments
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be identifier of an owned bot, or identifier of an owned channel chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_star_ad_account_url(owner_id: crate::enums::MessageSender, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getStarAdAccountUrl",
        "owner_id": owner_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
