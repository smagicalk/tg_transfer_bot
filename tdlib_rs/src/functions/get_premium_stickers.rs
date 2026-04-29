#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns premium stickers from regular sticker sets
/// # Arguments
/// * `limit` - The maximum number of stickers to be returned; 0-100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_stickers(limit: i32, client_id: i32) -> Result<crate::enums::Stickers, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumStickers",
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
