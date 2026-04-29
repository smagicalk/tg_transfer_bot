#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Loads an asynchronous or a zoomed in statistical graph
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `token` - The token for graph loading
/// * `x` - X-value for zoomed in graph or 0 otherwise
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_statistical_graph(chat_id: i64, token: String, x: i64, client_id: i32) -> Result<crate::enums::StatisticalGraph, crate::types::Error> {
    let request = json!({
        "@type": "getStatisticalGraph",
        "chat_id": chat_id,
        "token": token,
        "x": x,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
