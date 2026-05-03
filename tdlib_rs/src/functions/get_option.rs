use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the value of an option by its name. (Check the list of available options on https:core.telegram.org/tdlib/options.) Can be called before authorization. Can be called synchronously for options "version" and "commit_hash"
/// # Arguments
/// * `name` - The name of the option
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_option(
    name: String,
    client_id: i32,
) -> Result<crate::enums::OptionValue, crate::types::Error> {
    let request = json!({
    "@type": "getOption",
    "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
