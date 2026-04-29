#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about restrictions that must be applied to a chat or a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RestrictionInfo {
    /// A human-readable description of the reason why access to the content must be restricted. If empty, then the content can be accessed,
    /// but may be covered by hidden with 18+ spoiler anyway
    pub restriction_reason: String,
    /// True, if media content of the messages must be hidden with 18+ spoiler.
    /// Use value of the option "can_ignore_sensitive_content_restrictions" to check whether the current user can ignore the restriction.
    /// If age verification parameters were received in updateAgeVerificationParameters, then the user must complete age verification to ignore the restriction.
    /// Set the option "ignore_sensitive_content_restrictions" to true if the user passes age verification
    pub has_sensitive_content: bool,
}
