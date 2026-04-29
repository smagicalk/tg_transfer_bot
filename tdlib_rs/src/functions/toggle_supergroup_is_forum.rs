#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether the supergroup is a forum; requires owner privileges in the supergroup. Discussion supergroups can't be converted to forums
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup
/// * `is_forum` - New value of is_forum
/// * `has_forum_tabs` - New value of has_forum_tabs; ignored if is_forum is false
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_is_forum(supergroup_id: i64, is_forum: bool, has_forum_tabs: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupIsForum",
        "supergroup_id": supergroup_id,
        "is_forum": is_forum,
        "has_forum_tabs": has_forum_tabs,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
