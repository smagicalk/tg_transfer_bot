#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes a note of a contact user
/// # Arguments
/// * `user_id` - User identifier
/// * `note` - Note to set for the user; 0-getOption("user_note_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_user_note(user_id: i64, note: crate::types::FormattedText, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUserNote",
        "user_id": user_id,
        "note": note,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
