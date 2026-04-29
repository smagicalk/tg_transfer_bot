#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that a previously opened Web App was closed
/// # Arguments
/// * `web_app_launch_id` - Identifier of Web App launch, received from openWebApp
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn close_web_app(web_app_launch_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeWebApp",
        "web_app_launch_id": web_app_launch_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
