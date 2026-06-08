#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element containing the user's temporary registration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementTemporaryRegistration {
    /// Temporary registration
    pub temporary_registration: crate::types::PersonalDocument,
}
