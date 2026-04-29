#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a phone number by its prefix. Can be called before authorization
/// # Arguments
/// * `phone_number_prefix` - The phone number prefix
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_phone_number_info(phone_number_prefix: String, client_id: i32) -> Result<crate::enums::PhoneNumberInfo, crate::types::Error> {
    let request = json!({
        "@type": "getPhoneNumberInfo",
        "phone_number_prefix": phone_number_prefix,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
