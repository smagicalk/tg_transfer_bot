#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element to be saved containing the user's passport
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementPassport {
    /// The passport to be saved
    pub passport: crate::types::InputIdentityDocument,
}
