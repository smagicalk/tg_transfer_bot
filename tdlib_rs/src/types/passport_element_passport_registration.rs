#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element containing the user's passport registration pages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPassportRegistration {
    /// Passport registration pages
    pub passport_registration: crate::types::PersonalDocument,
}
