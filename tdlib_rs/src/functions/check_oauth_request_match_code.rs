use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks a match-code for an OAuth authorization request. If fails, then the authorization request has failed. Otherwise,
/// authorization confirmation dialog must be shown and the link must be processed using acceptOauthRequest or declineOauthRequest
/// # Arguments
/// * `url` - URL of the OAuth deep link
/// * `match_code` - The matching code chosen by the user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_oauth_request_match_code(
    url: String,
    match_code: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "checkOauthRequestMatchCode",
    "url": url,
    "match_code": match_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
