use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Updates the game score of the specified user in a game; for bots only
/// # Arguments
/// * `inline_message_id` - Inline message identifier
/// * `edit_message` - Pass true to edit the game message to include the current scoreboard
/// * `user_id` - User identifier
/// * `score` - The new score
/// * `force` - Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_inline_game_score(
    inline_message_id: String,
    edit_message: bool,
    user_id: i64,
    score: i32,
    force: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setInlineGameScore",
    "inline_message_id": inline_message_id,
    "edit_message": edit_message,
    "user_id": user_id,
    "score": score,
    "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
