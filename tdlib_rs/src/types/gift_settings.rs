#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains settings for gift receiving for a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftSettings {
    /// True, if a button for sending a gift to the user or by the user must always be shown in the input field
    pub show_gift_button: bool,
    /// Types of gifts accepted by the user; for Telegram Premium users only
    pub accepted_gift_types: crate::types::AcceptedGiftTypes,
}
