#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an HTTP URL which can be used to automatically authorize the current user on a website after clicking an HTTP link.
/// Use the method getExternalLinkInfo to find whether a prior user confirmation is needed. May return an empty link if just a toast about successful login has to be shown
/// # Arguments
/// * `link` - The HTTP link
/// * `allow_write_access` - Pass true if the current user allowed the bot that was returned in getExternalLinkInfo, to send them messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_external_link(link: String, allow_write_access: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getExternalLink",
        "link": link,
        "allow_write_access": allow_write_access,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
