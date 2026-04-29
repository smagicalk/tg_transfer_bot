#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Converts a themeParameters object to corresponding JSON-serialized string. Can be called synchronously
/// # Arguments
/// * `theme` - Theme parameters to convert to JSON
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_theme_parameters_json_string(theme: crate::types::ThemeParameters, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getThemeParametersJsonString",
        "theme": theme,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
