#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain
/// # Arguments
/// * `new_encryption_key` - New encryption key
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_database_encryption_key(new_encryption_key: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDatabaseEncryptionKey",
        "new_encryption_key": new_encryption_key,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
