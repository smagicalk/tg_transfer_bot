use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about the type of internal link. Returns a 404 error if the link is not internal. Can be called before authorization
/// # Arguments
/// * `link` - The link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_internal_link_type(
    link: String,
    client_id: i32,
) -> Result<crate::enums::InternalLinkType, crate::types::Error> {
    let request = json!({
    "@type": "getInternalLinkType",
    "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
