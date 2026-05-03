use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Accepts Telegram terms of services
/// # Arguments
/// * `terms_of_service_id` - Terms of service identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn accept_terms_of_service(
    terms_of_service_id: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "acceptTermsOfService",
    "terms_of_service_id": terms_of_service_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
