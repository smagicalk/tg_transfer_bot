#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a link preview by the text of a message. Do not call this function too often. Returns a 404 error if the text has no link preview
/// # Arguments
/// * `text` - Message text with formatting
/// * `link_preview_options` - Options to be used for generation of the link preview; pass null to use default link preview options
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_link_preview(text: crate::types::FormattedText, link_preview_options: Option<crate::types::LinkPreviewOptions>, client_id: i32) -> Result<crate::enums::LinkPreview, crate::types::Error> {
    let request = json!({
        "@type": "getLinkPreview",
        "text": text,
        "link_preview_options": link_preview_options,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
