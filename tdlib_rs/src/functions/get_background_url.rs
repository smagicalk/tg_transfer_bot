use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Constructs a persistent HTTP URL for a background
/// # Arguments
/// * `name` - Background name
/// * `r#type` - Background type; backgroundTypeChatTheme isn't supported
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_background_url(
    name: String,
    r#type: crate::enums::BackgroundType,
    client_id: i32,
) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
    "@type": "getBackgroundUrl",
    "name": name,
    "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
