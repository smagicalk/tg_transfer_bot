#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the mask position of a mask sticker. The sticker must belong to a mask sticker set that is owned by the current user
/// # Arguments
/// * `sticker` - Sticker
/// * `mask_position` - Position where the mask is placed; pass null to remove mask position
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_sticker_mask_position(sticker: crate::enums::InputFile, mask_position: Option<crate::types::MaskPosition>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setStickerMaskPosition",
        "sticker": sticker,
        "mask_position": mask_position,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
