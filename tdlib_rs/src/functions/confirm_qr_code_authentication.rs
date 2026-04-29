#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Confirms QR code authentication on another device. Returns created session on success
/// # Arguments
/// * `link` - A link from a QR code. The link must be scanned by the in-app camera
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn confirm_qr_code_authentication(link: String, client_id: i32) -> Result<crate::enums::Session, crate::types::Error> {
    let request = json!({
        "@type": "confirmQrCodeAuthentication",
        "link": link,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
