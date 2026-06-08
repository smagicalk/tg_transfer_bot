use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a gift to another user or channel chat. May return an error with a message "STARGIFT_USAGE_LIMITED" if the gift was sold out
/// # Arguments
/// * `gift_id` - Identifier of the gift to send
/// * `owner_id` - Identifier of the user or the channel chat that will receive the gift; limited gifts can't be sent to channel chats
/// * `text` - Text to show along with the gift; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed.
/// Must be empty if the receiver enabled paid messages
/// * `is_private` - Pass true to show gift text and sender only to the gift receiver; otherwise, everyone will be able to see them
/// * `pay_for_upgrade` - Pass true to additionally pay for the gift upgrade and allow the receiver to upgrade it for free
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_gift(
    gift_id: i64,
    owner_id: crate::enums::MessageSender,
    text: crate::types::FormattedText,
    is_private: bool,
    pay_for_upgrade: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "sendGift",
    "gift_id": gift_id,
    "owner_id": owner_id,
    "text": text,
    "is_private": is_private,
    "pay_for_upgrade": pay_for_upgrade,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
