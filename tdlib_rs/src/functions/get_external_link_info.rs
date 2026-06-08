use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about an action to be done when the current user clicks an external link. Don't use this method for links from secret chats if link preview is disabled in secret chats
/// # Arguments
/// * `link` - The link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_external_link_info(
    link: String,
    client_id: i32,
) -> Result<crate::enums::LoginUrlInfo, crate::types::Error> {
    let request = json!({
    "@type": "getExternalLinkInfo",
    "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
