#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a bot that can be added to attachment or side menu
/// # Arguments
/// * `bot_user_id` - Bot's user identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_attachment_menu_bot(bot_user_id: i64, client_id: i32) -> Result<crate::enums::AttachmentMenuBot, crate::types::Error> {
    let request = json!({
        "@type": "getAttachmentMenuBot",
        "bot_user_id": bot_user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
