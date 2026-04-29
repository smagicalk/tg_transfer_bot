#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets the value of an option. (Check the list of available options on https:core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization
/// # Arguments
/// * `name` - The name of the option
/// * `value` - The new value of the option; pass null to reset option value to a default value
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_option(name: String, value: Option<crate::enums::OptionValue>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setOption",
        "name": name,
        "value": value,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
