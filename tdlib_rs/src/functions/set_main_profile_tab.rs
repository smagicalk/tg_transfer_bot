use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the main profile tab of the current user
/// # Arguments
/// * `main_profile_tab` - The new value of the main profile tab
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_main_profile_tab(
    main_profile_tab: crate::enums::ProfileTab,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setMainProfileTab",
    "main_profile_tab": main_profile_tab,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
