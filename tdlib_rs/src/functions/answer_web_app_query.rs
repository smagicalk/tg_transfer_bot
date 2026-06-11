use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the result of interaction with a Web App and sends corresponding message on behalf of the user to the chat from which the query originated; for bots only
/// # Arguments
/// * `web_app_query_id` - Identifier of the Web App query
/// * `result` - The result of the query
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn answer_web_app_query(
    web_app_query_id: String,
    result: crate::enums::InputInlineQueryResult,
    client_id: i32,
) -> Result<crate::enums::SentWebAppMessage, crate::types::Error> {
    let request = json!({
    "@type": "answerWebAppQuery",
    "web_app_query_id": web_app_query_id,
    "result": result,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
