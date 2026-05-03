use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. Can be called synchronously
/// # Arguments
/// * `language_pack_database_path` - Path to the language pack database in which strings are stored
/// * `localization_target` - Localization target to which the language pack belongs
/// * `language_pack_id` - Language pack identifier
/// * `key` - Language pack key of the string to be returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_language_pack_string(
    language_pack_database_path: String,
    localization_target: String,
    language_pack_id: String,
    key: String,
    client_id: i32,
) -> Result<crate::enums::LanguagePackStringValue, crate::types::Error> {
    let request = json!({
    "@type": "getLanguagePackString",
    "language_pack_database_path": language_pack_database_path,
    "localization_target": localization_target,
    "language_pack_id": language_pack_id,
    "key": key,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
