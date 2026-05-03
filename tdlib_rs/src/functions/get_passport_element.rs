use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns one of the available Telegram Passport elements
/// # Arguments
/// * `r#type` - Telegram Passport element type
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_passport_element(
    r#type: crate::enums::PassportElementType,
    password: String,
    client_id: i32,
) -> Result<crate::enums::PassportElement, crate::types::Error> {
    let request = json!({
    "@type": "getPassportElement",
    "type": r#type,
    "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
