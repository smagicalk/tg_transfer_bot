use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks whether a name can be used for a new sticker set
/// # Arguments
/// * `name` - Name to be checked
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_sticker_set_name(
    name: String,
    client_id: i32,
) -> Result<crate::enums::CheckStickerSetNameResult, crate::types::Error> {
    let request = json!({
    "@type": "checkStickerSetName",
    "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
