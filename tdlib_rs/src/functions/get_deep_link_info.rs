#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a tg: deep link. Use "tg:need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization
/// # Arguments
/// * `link` - The link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_deep_link_info(link: String, client_id: i32) -> Result<crate::enums::DeepLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "getDeepLinkInfo",
        "link": link,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
