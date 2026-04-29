#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Passport element to be saved containing the user's identity card
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementIdentityCard {
    /// The identity card to be saved
    pub identity_card: crate::types::InputIdentityDocument,
}
