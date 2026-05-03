use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes a Telegram Passport element
/// # Arguments
/// * `r#type` - Element type
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_passport_element(
    r#type: crate::enums::PassportElementType,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deletePassportElement",
    "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
