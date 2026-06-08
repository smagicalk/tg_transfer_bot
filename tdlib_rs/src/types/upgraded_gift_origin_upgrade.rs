#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The gift was obtained by upgrading of a previously received gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginUpgrade {
    /// Identifier of the message with the regular gift that was upgraded; may be 0 or an identifier of a deleted message
    pub gift_message_id: i64,
}
