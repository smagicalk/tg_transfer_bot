#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The "Telegram Business" section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionBusiness {
    /// Subsection of the section; may be one of
    /// "", "do-not-hide-ads"
    pub subsection: String,
}
