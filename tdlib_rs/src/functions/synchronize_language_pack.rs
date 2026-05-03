use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Fetches the latest versions of all strings from a language pack in the current localization target from the server.
/// This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization
/// # Arguments
/// * `language_pack_id` - Language pack identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn synchronize_language_pack(
    language_pack_id: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "synchronizeLanguagePack",
    "language_pack_id": language_pack_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
