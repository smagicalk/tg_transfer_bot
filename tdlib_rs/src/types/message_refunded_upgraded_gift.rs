#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A gift which purchase, upgrade or transfer were refunded
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageRefundedUpgradedGift {
    /// The gift
    pub gift: crate::types::Gift,
    /// Sender of the gift
    pub sender_id: crate::enums::MessageSender,
    /// Receiver of the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Origin of the upgraded gift
    pub origin: crate::enums::UpgradedGiftOrigin,
}
