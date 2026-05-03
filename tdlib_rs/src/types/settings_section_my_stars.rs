#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The Telegram Star balance and transaction section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionMyStars {
    /// Subsection of the section; may be one of
    /// "", "top-up", "stats", "gift", "earn"
    pub subsection: String,
}
