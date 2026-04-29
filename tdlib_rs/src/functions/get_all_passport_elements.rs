#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns all available Telegram Passport elements
/// # Arguments
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_all_passport_elements(password: String, client_id: i32) -> Result<crate::enums::PassportElements, crate::types::Error> {
    let request = json!({
        "@type": "getAllPassportElements",
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
