#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An HTTP URL needs to be open
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LoginUrlInfoOpen {
    /// The URL to open
    pub url: String,
    /// True, if there is no need to show an ordinary open URL confirmation
    pub skip_confirmation: bool,
}
