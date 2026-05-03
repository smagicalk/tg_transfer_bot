use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a business chat link for the current account. Requires Telegram Business subscription. There can be up to getOption("business_chat_link_count_max") links created. Returns the created link
/// # Arguments
/// * `link_info` - Information about the link to create
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_business_chat_link(
    link_info: crate::types::InputBusinessChatLink,
    client_id: i32,
) -> Result<crate::enums::BusinessChatLink, crate::types::Error> {
    let request = json!({
    "@type": "createBusinessChatLink",
    "link_info": link_info,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
