#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element to be saved containing the user's passport registration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementPassportRegistration {
    /// The passport registration page to be saved
    pub passport_registration: crate::types::InputPersonalDocument,
}
