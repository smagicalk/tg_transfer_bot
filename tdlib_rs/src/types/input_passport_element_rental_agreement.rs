#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element to be saved containing the user's rental agreement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementRentalAgreement {
    /// The rental agreement to be saved
    pub rental_agreement: crate::types::InputPersonalDocument,
}
