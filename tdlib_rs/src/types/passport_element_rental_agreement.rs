#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A Telegram Passport element containing the user's rental agreement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementRentalAgreement {
    /// Rental agreement
    pub rental_agreement: crate::types::PersonalDocument,
}
