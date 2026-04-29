#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns an HTTPS link, which can be used to get information about the current user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_user_link(client_id: i32) -> Result<crate::enums::UserLink, crate::types::Error> {
    let request = json!({
        "@type": "getUserLink",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
