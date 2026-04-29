#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an instant view version of a web page if available. This is an offline method if only_local is true. Returns a 404 error if the web page has no instant view page
/// # Arguments
/// * `url` - The web page URL
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_web_page_instant_view(url: String, only_local: bool, client_id: i32) -> Result<crate::enums::WebPageInstantView, crate::types::Error> {
    let request = json!({
        "@type": "getWebPageInstantView",
        "url": url,
        "only_local": only_local,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
