#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes background from the list of installed backgrounds
/// # Arguments
/// * `background_id` - The background identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_installed_background(background_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeInstalledBackground",
        "background_id": background_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
