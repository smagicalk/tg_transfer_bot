#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization
/// # Arguments
/// * `language_pack_id` - Language pack identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_language_pack_info(language_pack_id: String, client_id: i32) -> Result<crate::enums::LanguagePackInfo, crate::types::Error> {
    let request = json!({
        "@type": "getLanguagePackInfo",
        "language_pack_id": language_pack_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
