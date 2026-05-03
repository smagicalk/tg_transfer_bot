use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Launches a prepaid giveaway
/// # Arguments
/// * `giveaway_id` - Unique identifier of the prepaid giveaway
/// * `parameters` - Giveaway parameters
/// * `winner_count` - The number of users to receive giveaway prize
/// * `star_count` - The number of Telegram Stars to be distributed through the giveaway; pass 0 for Telegram Premium giveaways
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn launch_prepaid_giveaway(
    giveaway_id: i64,
    parameters: crate::types::GiveawayParameters,
    winner_count: i32,
    star_count: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "launchPrepaidGiveaway",
    "giveaway_id": giveaway_id,
    "parameters": parameters,
    "winner_count": winner_count,
    "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
