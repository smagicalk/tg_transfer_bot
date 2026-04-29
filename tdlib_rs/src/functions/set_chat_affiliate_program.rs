#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes affiliate program for a bot
/// # Arguments
/// * `chat_id` - Identifier of the chat with an owned bot for which affiliate program is changed
/// * `parameters` - Parameters of the affiliate program; pass null to close the currently active program. If there is an active program, then commission and program duration can only be increased.
    /// If the active program is scheduled to be closed, then it can't be changed anymore
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_affiliate_program(chat_id: i64, parameters: Option<crate::types::AffiliateProgramParameters>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatAffiliateProgram",
        "chat_id": chat_id,
        "parameters": parameters,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
