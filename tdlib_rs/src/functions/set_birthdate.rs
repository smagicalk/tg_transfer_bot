use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the birthdate of the current user
/// # Arguments
/// * `birthdate` - The new value of the current user's birthdate; pass null to remove the birthdate
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_birthdate(
    birthdate: Option<crate::types::Birthdate>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBirthdate",
    "birthdate": birthdate,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
