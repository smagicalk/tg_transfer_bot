use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches a user by their phone number. Returns a 404 error if the user can't be found
/// # Arguments
/// * `phone_number` - Phone number to search for
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_user_by_phone_number(
    phone_number: String,
    only_local: bool,
    client_id: i32,
) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
    "@type": "searchUserByPhoneNumber",
    "phone_number": phone_number,
    "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
