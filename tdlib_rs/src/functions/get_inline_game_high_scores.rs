use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns game high scores and some part of the high score table in the range of the specified user; for bots only
/// # Arguments
/// * `inline_message_id` - Inline message identifier
/// * `user_id` - User identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_inline_game_high_scores(
    inline_message_id: String,
    user_id: i64,
    client_id: i32,
) -> Result<crate::enums::GameHighScores, crate::types::Error> {
    let request = json!({
    "@type": "getInlineGameHighScores",
    "inline_message_id": inline_message_id,
    "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
