use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about a link to boost a chat. Can be called for any internal link of the type internalLinkTypeChatBoost
/// # Arguments
/// * `url` - The link to boost a chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_link_info(
    url: String,
    client_id: i32,
) -> Result<crate::enums::ChatBoostLinkInfo, crate::types::Error> {
    let request = json!({
    "@type": "getChatBoostLinkInfo",
    "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
