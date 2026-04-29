#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks,
/// so it must be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics
/// # Arguments
/// * `r#type` - The new network type; pass null to set network type to networkTypeOther
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_network_type(r#type: Option<crate::enums::NetworkType>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setNetworkType",
        "type": r#type,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
