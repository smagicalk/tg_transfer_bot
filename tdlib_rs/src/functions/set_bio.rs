#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the bio of the current user
/// # Arguments
/// * `bio` - The new value of the user bio; 0-getOption("bio_length_max") characters without line feeds
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_bio(bio: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBio",
        "bio": bio,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
