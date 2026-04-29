#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds or changes a custom local language pack to the current localization target
/// # Arguments
/// * `info` - Information about the language pack. Language pack identifier must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization
/// * `strings` - Strings of the new language pack
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_custom_language_pack(info: crate::types::LanguagePackInfo, strings: Vec<crate::types::LanguagePackString>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCustomLanguagePack",
        "info": info,
        "strings": strings,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
