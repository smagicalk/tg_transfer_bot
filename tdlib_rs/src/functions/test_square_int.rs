use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the squared received number; for testing only. This is an offline method. Can be called before authorization
/// # Arguments
/// * `x` - Number to square
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn test_square_int(
    x: i32,
    client_id: i32,
) -> Result<crate::enums::TestInt, crate::types::Error> {
    let request = json!({
    "@type": "testSquareInt",
    "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
