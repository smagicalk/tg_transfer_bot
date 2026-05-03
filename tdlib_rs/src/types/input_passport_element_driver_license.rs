#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element to be saved containing the user's driver license
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementDriverLicense {
    /// The driver license to be saved
    pub driver_license: crate::types::InputIdentityDocument,
}
