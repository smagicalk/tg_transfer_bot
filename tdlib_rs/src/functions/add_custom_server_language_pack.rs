use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization
/// # Arguments
/// * `language_pack_id` - Identifier of a language pack to be added
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_custom_server_language_pack(
    language_pack_id: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "addCustomServerLanguagePack",
    "language_pack_id": language_pack_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
