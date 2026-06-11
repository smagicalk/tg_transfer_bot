use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only
/// # Arguments
/// * `chat_id` - The chat that contains the message with the game
/// * `message_id` - Identifier of the message
/// * `user_id` - User identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_game_high_scores(
    chat_id: i64,
    message_id: i64,
    user_id: i64,
    client_id: i32,
) -> Result<crate::enums::GameHighScores, crate::types::Error> {
    let request = json!({
    "@type": "getGameHighScores",
    "chat_id": chat_id,
    "message_id": message_id,
    "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
