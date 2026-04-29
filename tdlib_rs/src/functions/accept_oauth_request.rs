#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Accepts an OAuth authorization request. Returns an HTTP URL to open after successful authorization.
/// May return an empty link if just a toast about successful login has to be shown
/// # Arguments
/// * `url` - URL of the OAuth deep link
/// * `match_code` - The matching code chosen by the user
/// * `allow_write_access` - Pass true if the current user allowed the bot that was returned in getOauthLinkInfo, to send them messages
/// * `allow_phone_number_access` - Pass true if the current user allowed the bot that was returned in getOauthLinkInfo, to access their phone number
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn accept_oauth_request(url: String, match_code: String, allow_write_access: bool, allow_phone_number_access: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "acceptOauthRequest",
        "url": url,
        "match_code": match_code,
        "allow_write_access": allow_write_access,
        "allow_phone_number_access": allow_phone_number_access,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
