#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted.
/// Can be called before authorization
/// # Arguments
/// * `language_pack_id` - Identifier of the language pack to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_language_pack(language_pack_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteLanguagePack",
        "language_pack_id": language_pack_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
