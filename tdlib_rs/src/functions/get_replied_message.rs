#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns information about a non-bundled message that is replied by a given message. Also, returns the pinned message for messagePinMessage,
/// the game message for messageGameScore, the invoice message for messagePaymentSuccessful, the message with a previously set same background for messageChatSetBackground,
/// the giveaway message for messageGiveawayCompleted, the checklist message for messageChecklistTasksDone, messageChecklistTasksAdded, the message with suggested post information
/// for messageSuggestedPostApprovalFailed, messageSuggestedPostApproved, messageSuggestedPostDeclined, messageSuggestedPostPaid, messageSuggestedPostRefunded,
/// the message with the regular gift that was upgraded for messageUpgradedGift with origin of the type upgradedGiftOriginUpgrade,
/// the message with gift purchase offer for messageUpgradedGiftPurchaseOfferRejected,
/// the message with the request to disable content protection for messageChatHasProtectedContentToggled,
/// and the topic creation message for topic messages without non-bundled replied message. Returns a 404 error if the message doesn't exist
/// # Arguments
/// * `chat_id` - Identifier of the chat the message belongs to
/// * `message_id` - Identifier of the reply message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_replied_message(chat_id: i64, message_id: i64, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getRepliedMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
