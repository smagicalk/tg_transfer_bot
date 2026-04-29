#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that a bot was opened from the list of similar bots
/// # Arguments
/// * `bot_user_id` - Identifier of the original bot, which similar bots were requested
/// * `opened_bot_user_id` - Identifier of the opened bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn open_bot_similar_bot(bot_user_id: i64, opened_bot_user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openBotSimilarBot",
        "bot_user_id": bot_user_id,
        "opened_bot_user_id": opened_bot_user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
