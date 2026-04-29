#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Disables the currently enabled proxy. Can be called before authorization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn disable_proxy(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "disableProxy",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
