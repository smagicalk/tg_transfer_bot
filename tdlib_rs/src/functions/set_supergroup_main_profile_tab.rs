#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the main profile tab of the channel; requires can_change_info administrator right
/// # Arguments
/// * `supergroup_id` - Identifier of the channel
/// * `main_profile_tab` - The new value of the main profile tab
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_main_profile_tab(supergroup_id: i64, main_profile_tab: crate::enums::ProfileTab, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupMainProfileTab",
        "supergroup_id": supergroup_id,
        "main_profile_tab": main_profile_tab,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
