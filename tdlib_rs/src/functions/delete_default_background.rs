use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes default background for chats
/// # Arguments
/// * `for_dark_theme` - Pass true if the background is deleted for a dark theme
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_default_background(
    for_dark_theme: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteDefaultBackground",
    "for_dark_theme": for_dark_theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
