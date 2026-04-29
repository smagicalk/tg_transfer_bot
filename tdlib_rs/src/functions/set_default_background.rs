#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets default background for chats; adds the background to the list of installed backgrounds
/// # Arguments
/// * `background` - The input background to use; pass null to create a new filled background
/// * `r#type` - Background type; pass null to use the default type of the remote background; backgroundTypeChatTheme isn't supported
/// * `for_dark_theme` - Pass true if the background is set for a dark theme
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_default_background(background: Option<crate::enums::InputBackground>, r#type: Option<crate::enums::BackgroundType>, for_dark_theme: bool, client_id: i32) -> Result<crate::enums::Background, crate::types::Error> {
    let request = json!({
        "@type": "setDefaultBackground",
        "background": background,
        "type": r#type,
        "for_dark_theme": for_dark_theme,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
