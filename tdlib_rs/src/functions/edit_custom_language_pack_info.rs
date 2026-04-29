#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Edits information about a custom local language pack in the current localization target. Can be called before authorization
/// # Arguments
/// * `info` - New information about the custom local language pack
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_custom_language_pack_info(info: crate::types::LanguagePackInfo, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editCustomLanguagePackInfo",
        "info": info,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
