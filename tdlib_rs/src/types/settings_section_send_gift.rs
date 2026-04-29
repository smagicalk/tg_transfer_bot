#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The "Send a gift" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionSendGift {
    /// Subsection of the section; may be one of
    /// "", "self"
    pub subsection: String,
}
