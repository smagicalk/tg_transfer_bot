#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An authentication code is a phrase from multiple words delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeSmsPhrase {
    /// The first word of the phrase if known
    pub first_word: String,
}
