use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns strings from a language pack in the current localization target by their keys. Can be called before authorization
/// # Arguments
/// * `language_pack_id` - Language pack identifier of the strings to be returned
/// * `keys` - Language pack keys of the strings to be returned; leave empty to request all available strings
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_language_pack_strings(
    language_pack_id: String,
    keys: Vec<String>,
    client_id: i32,
) -> Result<crate::enums::LanguagePackStrings, crate::types::Error> {
    let request = json!({
    "@type": "getLanguagePackStrings",
    "language_pack_id": language_pack_id,
    "keys": keys,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
