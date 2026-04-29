#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends a call rating
/// # Arguments
/// * `call_id` - Call identifier
/// * `rating` - Call rating; 1-5
/// * `comment` - An optional user comment if the rating is less than 5
/// * `problems` - List of the exact types of problems with the call, specified by the user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_call_rating(call_id: crate::enums::InputCall, rating: i32, comment: String, problems: Vec<crate::enums::CallProblem>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallRating",
        "call_id": call_id,
        "rating": rating,
        "comment": comment,
        "problems": problems,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
