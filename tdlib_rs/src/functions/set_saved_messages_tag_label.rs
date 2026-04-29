#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes label of a Saved Messages tag; for Telegram Premium users only
/// # Arguments
/// * `tag` - The tag which label will be changed
/// * `label` - New label for the tag; 0-12 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_saved_messages_tag_label(tag: crate::enums::ReactionType, label: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSavedMessagesTagLabel",
        "tag": tag,
        "label": label,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
