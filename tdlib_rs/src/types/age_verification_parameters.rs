#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes parameters for age verification of the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AgeVerificationParameters {
    /// The minimum age required to view restricted content
    pub min_age: i32,
    /// Username of the bot which main Web App may be used to verify age of the user
    pub verification_bot_username: String,
    /// Unique name for the country or region, which legislation required age verification. May be used to get the corresponding localization key
    pub country: String,
}
