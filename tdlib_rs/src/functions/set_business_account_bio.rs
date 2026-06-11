use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the bio of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `bio` - The new value of the bio; 0-getOption("bio_length_max") characters without line feeds
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_bio(
    business_connection_id: String,
    bio: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessAccountBio",
    "business_connection_id": business_connection_id,
    "bio": bio,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
