#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link.
/// If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePremiumGiftCode {
    /// The Telegram Premium gift code
    pub code: String,
}
