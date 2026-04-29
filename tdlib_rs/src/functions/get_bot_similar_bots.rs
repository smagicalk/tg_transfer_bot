#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a list of bots similar to the given bot
/// # Arguments
/// * `bot_user_id` - User identifier of the target bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_similar_bots(bot_user_id: i64, client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "getBotSimilarBots",
        "bot_user_id": bot_user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
