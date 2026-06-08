use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about an OAuth deep link. Use checkOauthRequestMatchCode, acceptOauthRequest or declineOauthRequest to process the link
/// # Arguments
/// * `url` - URL of the link
/// * `in_app_origin` - Origin of the OAuth request if the request was received from the in-app browser; pass an empty string otherwise
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_oauth_link_info(
    url: String,
    in_app_origin: String,
    client_id: i32,
) -> Result<crate::enums::OauthLinkInfo, crate::types::Error> {
    let request = json!({
    "@type": "getOauthLinkInfo",
    "url": url,
    "in_app_origin": in_app_origin,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
