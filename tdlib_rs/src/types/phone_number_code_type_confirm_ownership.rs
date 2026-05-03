#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Confirms ownership of a phone number to prevent account deletion while handling links of the type internalLinkTypePhoneNumberConfirmation
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberCodeTypeConfirmOwnership {
    /// Hash value from the link
    pub hash: String,
}
