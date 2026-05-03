use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a public or private message link. Can be called for any internal link of the type internalLinkTypeMessage
/// # Arguments
/// * `url` - The message link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_link_info(
    url: String,
    client_id: i32,
) -> Result<crate::enums::MessageLinkInfo, crate::types::Error> {
    let request = json!({
    "@type": "getMessageLinkInfo",
    "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
