use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns backgrounds installed by the user
/// # Arguments
/// * `for_dark_theme` - Pass true to order returned backgrounds for a dark theme
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_installed_backgrounds(
    for_dark_theme: bool,
    client_id: i32,
) -> Result<crate::enums::Backgrounds, crate::types::Error> {
    let request = json!({
    "@type": "getInstalledBackgrounds",
    "for_dark_theme": for_dark_theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
