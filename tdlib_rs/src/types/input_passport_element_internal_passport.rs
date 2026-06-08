#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element to be saved containing the user's internal passport
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementInternalPassport {
    /// The internal passport to be saved
    pub internal_passport: crate::types::InputIdentityDocument,
}
