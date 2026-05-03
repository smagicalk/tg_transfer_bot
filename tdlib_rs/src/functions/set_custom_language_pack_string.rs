use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds, edits or deletes a string in a custom local language pack. Can be called before authorization
/// # Arguments
/// * `language_pack_id` - Identifier of a previously added custom local language pack in the current localization target
/// * `new_string` - New language pack string
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_custom_language_pack_string(
    language_pack_id: String,
    new_string: crate::types::LanguagePackString,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setCustomLanguagePackString",
    "language_pack_id": language_pack_id,
    "new_string": new_string,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
