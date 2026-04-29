#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the current state of stake dice
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_stake_dice_state(client_id: i32) -> Result<crate::enums::StakeDiceState, crate::types::Error> {
    let request = json!({
        "@type": "getStakeDiceState",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
