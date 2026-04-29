#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of passkeys allowed to be used for the login by the current user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_login_passkeys(client_id: i32) -> Result<crate::enums::Passkeys, crate::types::Error> {
    let request = json!({
        "@type": "getLoginPasskeys",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
