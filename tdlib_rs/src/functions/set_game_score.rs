use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Updates the game score of the specified user in the game; for bots only
/// # Arguments
/// * `chat_id` - The chat to which the message with the game belongs
/// * `message_id` - Identifier of the message
/// * `edit_message` - Pass true to edit the game message to include the current scoreboard
/// * `user_id` - User identifier
/// * `score` - The new score
/// * `force` - Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_game_score(
    chat_id: i64,
    message_id: i64,
    edit_message: bool,
    user_id: i64,
    score: i32,
    force: bool,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "setGameScore",
    "chat_id": chat_id,
    "message_id": message_id,
    "edit_message": edit_message,
    "user_id": user_id,
    "score": score,
    "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
