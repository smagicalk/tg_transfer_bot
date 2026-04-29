#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentUpgradedGift {
    /// True, if the gift was obtained by upgrading of a previously received gift; otherwise, if is_prepaid_upgrade == false, then this is a transferred or resold gift
    pub is_upgrade: bool,
    /// True, if the message is about completion of prepaid upgrade of the gift instead of actual receiving of a new gift
    pub is_prepaid_upgrade: bool,
}
