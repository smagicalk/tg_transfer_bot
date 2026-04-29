#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an HTTPS or a tg: link with the given type. Can be called before authorization
/// # Arguments
/// * `r#type` - Expected type of the link
/// * `is_http` - Pass true to create an HTTPS link (only available for some link types); pass false to create a tg: link
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_internal_link(r#type: crate::enums::InternalLinkType, is_http: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getInternalLink",
        "type": r#type,
        "is_http": is_http,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
