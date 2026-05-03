use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn test_use_update(client_id: i32) -> Result<crate::enums::Update, crate::types::Error> {
    let request = json!({
    "@type": "testUseUpdate",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
