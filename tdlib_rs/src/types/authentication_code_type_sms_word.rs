#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An authentication code is a word delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeSmsWord {
    /// The first letters of the word if known
    pub first_letter: String,
}
