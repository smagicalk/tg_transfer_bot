#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Hides the list of contacts that have close birthdays for 24 hours
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn hide_contact_close_birthdays(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "hideContactCloseBirthdays",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
