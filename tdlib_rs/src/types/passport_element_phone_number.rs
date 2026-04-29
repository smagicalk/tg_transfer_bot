#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Passport element containing the user's phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPhoneNumber {
    /// Phone number
    pub phone_number: String,
}
