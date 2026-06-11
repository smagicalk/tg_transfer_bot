use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Answers a custom query; for bots only
/// # Arguments
/// * `custom_query_id` - Identifier of a custom query
/// * `data` - JSON-serialized answer to the query
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn answer_custom_query(
    custom_query_id: i64,
    data: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "answerCustomQuery",
    "custom_query_id": custom_query_id,
    "data": data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
