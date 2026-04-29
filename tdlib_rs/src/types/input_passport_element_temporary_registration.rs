#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Passport element to be saved containing the user's temporary registration
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementTemporaryRegistration {
    /// The temporary registration document to be saved
    pub temporary_registration: crate::types::InputPersonalDocument,
}
