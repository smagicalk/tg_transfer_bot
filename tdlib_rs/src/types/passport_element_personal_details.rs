#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Passport element containing the user's personal details
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPersonalDetails {
    /// Personal details of the user
    pub personal_details: crate::types::PersonalDetails,
}
